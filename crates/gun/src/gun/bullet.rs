use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{Bullet, Damage, Hp};

const BULLET_SIZE: f32 = 1. / 8.;

/// A marker component for a bullet shot by a Gun
#[derive(Component)]
pub struct Common {
    owner: Entity,
    bounce_count: usize,
}

impl Common {
    pub fn new(owner: Entity) -> Self {
        Self {
            owner,
            bounce_count: 0,
        }
    }
}

impl Bullet for Common {
    fn shoot(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        origin: Vec3,
        force: Vec3,
        owner: Entity,
    ) {
        commands.spawn((
            Transform::from_translation(origin),
            Mesh3d(meshes.add(Sphere::new(BULLET_SIZE).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            })),
            RigidBody::Dynamic,
            Collider::sphere(BULLET_SIZE),
            SweptCcd::default(),
            LinearVelocity(force),
            Mass(3.0),
            CollisionEventsEnabled,
            Common::new(owner),
            Hp::ammo(),
        ));
    }

    fn gunfire_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, place: Vec3) {
        commands.spawn((
            Transform::from_translation(place),
            AudioPlayer::new(asset_server.load("SE/shoot.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));
    }

    fn owner(&self) -> Entity {
        self.owner
    }

    fn bounce_count(&self) -> usize {
        self.bounce_count
    }
}

/// Whether `entity` is `owner` itself or a descendant of it (e.g. a gun or other piece of
/// equipment parented under the player that owns it).
fn is_owned_by(entity: Entity, owner: Entity, parent_query: &Query<&ChildOf>) -> bool {
    let mut current = entity;
    loop {
        if current == owner {
            return true;
        }
        match parent_query.get(current) {
            Ok(child_of) => current = child_of.parent(),
            Err(_) => return false,
        }
    }
}

// Bullet specific collision system
pub fn bullet_collision_system(
    mut commands: Commands,
    mut collision_event_reader: MessageReader<CollisionStart>,
    mut bullet_query: Query<(&mut Common, &LinearVelocity, &Mass)>,
    other_query: Query<(Option<&LinearVelocity>, Option<&Mass>)>,
    parent_query: Query<&ChildOf>,
) {
    for event in collision_event_reader.read() {
        let e1 = event.collider1;
        let e2 = event.collider2;

        let mut process_collision = |bullet_entity, other_entity| {
            if let Ok((mut bullet, b_vel, b_mass)) = bullet_query.get_mut(bullet_entity) {
                // Ignore the owner and the owner's own equipment (e.g. the gun that just fired
                // this bullet) until the bullet has bounced at least once.
                if bullet.bounce_count == 0
                    && is_owned_by(other_entity, bullet.owner, &parent_query)
                {
                    return;
                }

                // Calculate total speed
                let mut speed = b_vel.length();
                if let Ok((Some(o_vel), _)) = other_query.get(other_entity) {
                    speed += o_vel.length();
                }

                let damage = speed * **b_mass;

                // Apply damage to the hit object and the bullet itself
                commands.trigger(Damage {
                    target: other_entity,
                    amount: damage,
                });
                commands.trigger(Damage {
                    target: bullet_entity,
                    amount: damage,
                });

                // Increment bounce count
                bullet.bounce_count = bullet.bounce_count.saturating_add(1);
            }
        };

        process_collision(e1, e2);
        process_collision(e2, e1);
    }
}

#[cfg(test)]
mod tests {
    /// `Common::shoot`'s unit tests
    mod shoot {
        use super::super::{BULLET_SIZE, Common};
        use avian3d::prelude::*;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::Bullet;

        /// The spawned bullet's collider is sized to match its visual mesh.
        #[test]
        fn collider_matches_the_visual_size() {
            let mut world = World::new();
            world.insert_resource(Assets::<Mesh>::default());
            world.insert_resource(Assets::<StandardMaterial>::default());

            world
                .run_system_once(
                    |mut commands: Commands,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<StandardMaterial>>| {
                        Common::shoot(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            Vec3::ZERO,
                            Vec3::NEG_Z,
                            Entity::PLACEHOLDER,
                        );
                    },
                )
                .unwrap();

            let collider = world.query::<&Collider>().single(&world).unwrap();
            let radius = collider.shape().as_ball().unwrap().radius;
            assert_eq!(radius, BULLET_SIZE);
        }
    }

    /// `bullet_collision_system`'s unit tests
    mod bullet_collision_system {
        use super::super::{Common, bullet_collision_system};
        use avian3d::prelude::*;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::{Bullet, Damage};

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

        fn spawn_bullet(world: &mut World, owner: Entity) -> Entity {
            world
                .spawn((
                    Common::new(owner),
                    LinearVelocity(Vec3::NEG_Z * 500.),
                    Mass(3.0),
                ))
                .id()
        }

        /// A bullet colliding with its own gun (a child of its owner) on the first bounce takes
        /// no self-damage, instead of despawning on the frame it was fired.
        #[test]
        fn ignores_collision_with_the_owners_equipment_on_first_bounce() {
            let mut world = world_with_damage_log();
            let owner = world.spawn_empty().id();
            let gun = world.spawn(ChildOf(owner)).id();
            let bullet = spawn_bullet(&mut world, owner);
            write_collision(&mut world, bullet, gun);

            world.run_system_once(bullet_collision_system).unwrap();

            assert!(world.resource::<DamageLog>().0.is_empty());
            assert_eq!(world.get::<Common>(bullet).unwrap().bounce_count(), 0);
        }

        /// A bullet colliding with its owner directly on the first bounce takes no self-damage.
        #[test]
        fn ignores_collision_with_the_owner_on_first_bounce() {
            let mut world = world_with_damage_log();
            let owner = world.spawn_empty().id();
            let bullet = spawn_bullet(&mut world, owner);
            write_collision(&mut world, bullet, owner);

            world.run_system_once(bullet_collision_system).unwrap();

            assert!(world.resource::<DamageLog>().0.is_empty());
            assert_eq!(world.get::<Common>(bullet).unwrap().bounce_count(), 0);
        }

        /// A real hit against an unrelated entity damages both sides and advances bounce_count.
        #[test]
        fn applies_damage_on_a_real_hit_and_advances_bounce_count() {
            let mut world = world_with_damage_log();
            let owner = world.spawn_empty().id();
            let bullet = spawn_bullet(&mut world, owner);
            let target = world.spawn(LinearVelocity(Vec3::ZERO)).id();
            write_collision(&mut world, bullet, target);

            world.run_system_once(bullet_collision_system).unwrap();

            let log = &world.resource::<DamageLog>().0;
            assert_eq!(log.len(), 2);
            assert!(log.iter().any(|(e, _)| *e == target));
            assert!(log.iter().any(|(e, _)| *e == bullet));
            assert_eq!(world.get::<Common>(bullet).unwrap().bounce_count(), 1);
        }

        /// Once a bullet has already bounced, a further collision with its owner is a real hit.
        #[test]
        fn applies_damage_against_the_owner_after_the_first_bounce() {
            let mut world = world_with_damage_log();
            let owner = world.spawn_empty().id();
            let bullet = world
                .spawn((
                    Common {
                        owner,
                        bounce_count: 1,
                    },
                    LinearVelocity(Vec3::NEG_Z * 500.),
                    Mass(3.0),
                ))
                .id();
            write_collision(&mut world, bullet, owner);

            world.run_system_once(bullet_collision_system).unwrap();

            let log = &world.resource::<DamageLog>().0;
            assert!(log.iter().any(|(e, _)| *e == owner));
            assert_eq!(world.get::<Common>(bullet).unwrap().bounce_count(), 2);
        }
    }
}
