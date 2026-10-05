//! # Gun systems, components & etc...

pub mod bullet;
pub mod select_fire;

use self::select_fire::SelectFire;
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{Damage, configs::GameConfigs};
use spacerobo_target::Common as CommonTarget;

/// Rounds a freshly reloaded gun holds.
///
/// Prototype value for feeling out the ammo/reload loop (see issue discussion); not wired to
/// `GameConfigs` yet. The actual ammo count lives in `spacerobo_commons::Ammo`, on the gun's
/// owner, not on `Gun` itself, so `spacerobo_hud` can display it without depending on this crate.
pub const MAGAZINE_SIZE: u32 = 30;

/// Gun component
#[derive(Component)]
pub struct Gun {
    pub owner: Entity,

    /// Select fire setting
    pub select_fire: SelectFire,

    /// A interval settings and values
    pub interval: Interval,
}

impl Gun {
    pub fn spawn_as_child(
        parent: &mut ChildSpawnerCommands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        origin: Vec3,
        game_configs: &GameConfigs,
    ) {
        const DEFAULT_FIREMODE: SelectFire = SelectFire::Full;

        parent
            .spawn((
                Transform::from_translation(origin),
                Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                (Gun {
                    owner: parent.target_entity(),
                    select_fire: DEFAULT_FIREMODE,
                    interval: Interval {
                        limit: game_configs.player.robo.gun.interval_limit,
                        rest: 0.0,
                        amount: game_configs.player.robo.gun.interval_amount,
                    },
                }),
                ColliderConstructor::ConvexHullFromMesh,
                CollisionEventsEnabled,
            ))
            // Spot light
            .with_child((
                SpotLight {
                    intensity: 100_000_000.0,
                    range: 100_000_000.0,
                    outer_angle: std::f32::consts::FRAC_PI_4 / 2.0,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, -1.3).looking_to(Vec3::NEG_Z, Vec3::ZERO),
            ))
            // Muzzle
            .with_child((Transform::from_xyz(0.0, 0.0, -1.3), Muzzle));
    }
}

impl Gun {
    fn fullauto(&mut self) {
        self.select_fire = SelectFire::Full;
    }

    fn semiauto(&mut self) {
        self.select_fire = SelectFire::Semi;
    }
}

/// A interval settings and values
#[derive(Default)]
pub struct Interval {
    /// The upper limit of interval
    pub limit: f32,

    /// The rest of full-auto interval
    pub rest: f32,

    /// A number for rest_interval decrementing
    pub amount: f32,
}

/// A marker component to know muzzle's transform
#[derive(Component)]
pub struct Muzzle;

/// Gun cooling system.
/// It controls full auto's shoot interval.
pub fn gun_cooling_system(mut gun: Query<&mut Gun>) {
    for mut gun in gun.iter_mut() {
        gun.interval.rest -= gun.interval.amount;
    }
}

/// Melee damage dealt to a target a gun collides with.
const HUGE_DAMAGE: f32 = 20000.0;

pub fn gun_melee_damage_system(
    mut commands: Commands,
    mut collision_event_reader: MessageReader<CollisionStart>,
    gun_query: Query<(), With<Gun>>,
    target_query: Query<(), With<CommonTarget>>,
) {
    for event in collision_event_reader.read() {
        debug!("Collision!!");

        let e1 = event.collider1;
        let e2 = event.collider2;

        // Check which entity is the target when the gun collides
        let target_entity = if gun_query.contains(e1) {
            e2
        } else if gun_query.contains(e2) {
            e1
        } else {
            continue;
        };

        if target_query.contains(target_entity) {
            commands.trigger(Damage {
                target: target_entity,
                amount: HUGE_DAMAGE,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    /// `Gun::spawn_as_child`'s unit tests
    mod spawn_as_child {
        use super::super::Gun;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::configs::GameConfigs;

        /// The spawned gun's fire-rate interval comes from `GameConfigs`, not a hardcoded value.
        #[test]
        fn interval_comes_from_game_configs() {
            let mut world = World::new();
            world.insert_resource(Assets::<Mesh>::default());
            world.insert_resource(Assets::<StandardMaterial>::default());

            let mut game_configs = GameConfigs::default();
            game_configs.player.robo.gun.interval_limit = 0.25;
            game_configs.player.robo.gun.interval_amount = 0.05;

            world
                .run_system_once(
                    move |mut commands: Commands,
                          mut meshes: ResMut<Assets<Mesh>>,
                          mut materials: ResMut<Assets<StandardMaterial>>| {
                        commands.spawn_empty().with_children(|parent| {
                            Gun::spawn_as_child(
                                parent,
                                &mut meshes,
                                &mut materials,
                                Vec3::ZERO,
                                &game_configs,
                            );
                        });
                    },
                )
                .unwrap();

            let gun = world.query::<&Gun>().single(&world).unwrap();
            assert_eq!(gun.interval.limit, 0.25);
            assert_eq!(gun.interval.amount, 0.05);
            assert_eq!(gun.interval.rest, 0.0);
        }
    }

    /// `gun_melee_damage_system`'s unit tests
    mod gun_melee_damage_system {
        use super::super::{Gun, HUGE_DAMAGE, Interval, gun_melee_damage_system};
        use avian3d::prelude::*;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::Damage;
        use spacerobo_target::Common as CommonTarget;

        #[derive(Resource, Default)]
        struct DamageLog(Vec<(Entity, f32)>);

        fn record_damage(damage: On<Damage>, mut log: ResMut<DamageLog>) {
            log.0.push((damage.target, damage.amount));
        }

        fn world_with_damage_log() -> World {
            let mut world = World::new();
            world.insert_resource(DamageLog::default());
            world.add_observer(record_damage);
            world.init_resource::<Messages<CollisionStart>>();
            world
        }

        fn write_collision(world: &mut World, collider1: Entity, collider2: Entity) {
            world.write_message(CollisionStart {
                collider1,
                collider2,
                body1: None,
                body2: None,
            });
        }

        fn spawn_gun(world: &mut World) -> Entity {
            world
                .spawn(Gun {
                    owner: Entity::PLACEHOLDER,
                    select_fire: Default::default(),
                    interval: Interval::default(),
                })
                .id()
        }

        /// Colliding with a `CommonTarget` triggers melee `Damage` against it.
        #[test]
        fn triggers_damage_against_a_target_on_collision() {
            let mut world = world_with_damage_log();
            let gun = spawn_gun(&mut world);
            let target = world.spawn(CommonTarget).id();
            write_collision(&mut world, gun, target);

            world.run_system_once(gun_melee_damage_system).unwrap();

            assert_eq!(world.resource::<DamageLog>().0, vec![(target, HUGE_DAMAGE)]);
        }

        /// Colliding with a non-target entity deals no damage.
        #[test]
        fn does_not_trigger_damage_against_a_non_target() {
            let mut world = world_with_damage_log();
            let gun = spawn_gun(&mut world);
            let other = world.spawn_empty().id();
            write_collision(&mut world, gun, other);

            world.run_system_once(gun_melee_damage_system).unwrap();

            assert!(world.resource::<DamageLog>().0.is_empty());
        }

        /// A collision between two non-gun entities is ignored.
        #[test]
        fn ignores_collisions_without_a_gun() {
            let mut world = world_with_damage_log();
            let a = world.spawn(CommonTarget).id();
            let b = world.spawn(CommonTarget).id();
            write_collision(&mut world, a, b);

            world.run_system_once(gun_melee_damage_system).unwrap();

            assert!(world.resource::<DamageLog>().0.is_empty());
        }
    }
}
