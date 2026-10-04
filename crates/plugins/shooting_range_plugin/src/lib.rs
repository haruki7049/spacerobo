use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};
use spacerobo_commons::{Damage, DeathMessage, GameMode, Hp, KillCounter, Target};
use spacerobo_player::PlayerCommonPlugin;
use spacerobo_target::Common as CommonTarget;

/// The world extends this far from the origin on every axis, in both directions. Entities that
/// cross it are despawned (`when_going_outside_system`), and the visible grid (`spawn_boundary_grid`)
/// is drawn at this same distance, so the two stay in sync.
const WORLD_BOUNDARY_LIMIT: f32 = 2000.0;

pub struct ShootingRangePlugin;

impl Plugin for ShootingRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCommonPlugin);
        app.insert_resource(Gravity(Vec3::NEG_Y * 0.));
        app.add_systems(
            OnEnter(GameMode::InGame),
            (setup_system, spawn_boundary_grid).run_if(in_state(GameMode::InGame)),
        );
        app.add_systems(
            Update,
            (
                // Systems
                when_going_outside_system,
                death_system,
            )
                .run_if(in_state(GameMode::InGame)),
        );
        app.add_observer(apply_damage_system);
    }
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // Light
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 2.0),
    ));

    // One entry per octant: the sign applied to each axis, and that octant's target color.
    let octants: [(Vec3, Color); 8] = [
        (Vec3::new(1., 1., 1.), RED.into()),
        (Vec3::new(1., 1., -1.), WHITE.into()),
        (Vec3::new(1., -1., 1.), WHITE.into()),
        (Vec3::new(1., -1., -1.), GREEN.into()),
        (Vec3::new(-1., 1., 1.), WHITE.into()),
        (Vec3::new(-1., 1., -1.), YELLOW.into()),
        (Vec3::new(-1., -1., 1.), BLUE.into()),
        (Vec3::new(-1., -1., -1.), WHITE.into()),
    ];

    // Targets
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                let i_float = i as f32;
                let j_float = j as f32;
                let k_float = k as f32;

                for (sign, color) in octants {
                    CommonTarget::spawn(
                        &mut commands,
                        &mut meshes,
                        &asset_server,
                        &mut materials,
                        color,
                        Vec3::new(i_float, j_float, k_float) * sign * 10.0,
                    );
                }
            }
        }
    }
}

fn when_going_outside_system(
    query: Query<(&Transform, Entity), With<Hp>>,
    mut event_writer: MessageWriter<DeathMessage>,
) {
    for (transform, entity) in query.iter() {
        if transform.translation.abs().max_element() > WORLD_BOUNDARY_LIMIT {
            debug!("Creating DeathMessage by area outside...");
            event_writer.write(DeathMessage::new(entity));
        }
    }
}

pub fn death_system(
    mut commands: Commands,
    mut event_reader: MessageReader<DeathMessage>,
    hp_query: Query<(&Hp, Option<&CommonTarget>)>,
    mut kill_counter: ResMut<KillCounter>,
) {
    for death_event in event_reader.read() {
        if let Ok((hp, target)) = hp_query.get(death_event.entity) {
            commands.entity(death_event.entity).despawn();
            if let Some(handle) = hp.death_sound.clone() {
                commands.spawn(AudioPlayer::new(handle));
            }

            if target.is_some() {
                kill_counter.increment();
            }

            debug!("{:?} which has Hp component is dead!!", death_event.entity);
        }
    }
}

pub fn spawn_boundary_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let limit = WORLD_BOUNDARY_LIMIT;
    let spacing = 200.0;
    let thickness = 2.0;

    // Mesh for horizontal and vertical lines
    let line_mesh_x = meshes.add(Cuboid::new(limit * 2.0, thickness, thickness));
    let line_mesh_y = meshes.add(Cuboid::new(thickness, limit * 2.0, thickness));

    let faces = [
        (Vec3::new(0.0, 0.0, limit), Quat::IDENTITY),
        (Vec3::new(0.0, 0.0, -limit), Quat::IDENTITY),
        (
            Vec3::new(limit, 0.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(-limit, 0.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(0.0, limit, 0.0),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(0.0, -limit, 0.0),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        ),
    ];

    for (face_pos, face_rot) in faces {
        // Create a unique material for each face
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.0, 0.0, 0.8),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        commands
            .spawn((
                Transform::from_translation(face_pos).with_rotation(face_rot),
                Visibility::default(),
            ))
            .with_children(|parent| {
                let mut i = -limit;
                while i <= limit {
                    // Horizontal line
                    parent.spawn((
                        Mesh3d(line_mesh_x.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(0.0, i, 0.0),
                    ));
                    // Vertical line
                    parent.spawn((
                        Mesh3d(line_mesh_y.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(i, 0.0, 0.0),
                    ));
                    i += spacing;
                }
            });
    }
}

fn apply_damage_system(
    damage: On<Damage>,
    mut query: Query<&mut Hp>,
    mut event_writer: MessageWriter<DeathMessage>,
) {
    if let Ok(mut hp) = query.get_mut(damage.target) {
        if hp.rest <= 0. {
            return;
        }

        hp.decrease(damage.amount);

        if hp.rest <= 0. {
            event_writer.write(DeathMessage::new(damage.target));
        }
    }
}

#[cfg(test)]
mod tests {
    /// `when_going_outside_system`'s unit tests
    mod when_going_outside_system {
        use super::super::when_going_outside_system;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::{DeathMessage, Hp};

        fn world_with_death_messages() -> World {
            let mut world = World::new();
            world.init_resource::<Messages<DeathMessage>>();
            world
        }

        fn spawn_at(world: &mut World, position: Vec3) -> Entity {
            world
                .spawn((Transform::from_translation(position), Hp::default()))
                .id()
        }

        /// Entities marked dead by the system, in the order their `DeathMessage` was written.
        fn died(world: &mut World) -> Vec<Entity> {
            world
                .run_system_once(|mut reader: MessageReader<DeathMessage>| {
                    reader
                        .read()
                        .map(|message| message.entity)
                        .collect::<Vec<_>>()
                })
                .unwrap()
        }

        /// An entity well within the boundary is left alone.
        #[test]
        fn leaves_entities_within_bounds_alone() {
            let mut world = world_with_death_messages();
            spawn_at(&mut world, Vec3::ZERO);

            world.run_system_once(when_going_outside_system).unwrap();

            assert!(died(&mut world).is_empty());
        }

        /// An entity exactly at the boundary is left alone (the check is a strict `>`).
        #[test]
        fn leaves_entities_exactly_at_the_boundary_alone() {
            let mut world = world_with_death_messages();
            spawn_at(&mut world, Vec3::new(2000.0, 2000.0, 2000.0));

            world.run_system_once(when_going_outside_system).unwrap();

            assert!(died(&mut world).is_empty());
        }

        /// An entity just beyond the positive bound on any single axis is marked dead.
        #[test]
        fn marks_entities_beyond_each_positive_bound_as_dead() {
            let mut world = world_with_death_messages();
            let x = spawn_at(&mut world, Vec3::new(2000.1, 0., 0.));
            let y = spawn_at(&mut world, Vec3::new(0., 2000.1, 0.));
            let z = spawn_at(&mut world, Vec3::new(0., 0., 2000.1));

            world.run_system_once(when_going_outside_system).unwrap();

            let dead = died(&mut world);
            assert_eq!(dead.len(), 3);
            assert!(dead.contains(&x));
            assert!(dead.contains(&y));
            assert!(dead.contains(&z));
        }

        /// An entity just beyond the negative bound on any single axis is marked dead.
        #[test]
        fn marks_entities_beyond_each_negative_bound_as_dead() {
            let mut world = world_with_death_messages();
            let x = spawn_at(&mut world, Vec3::new(-2000.1, 0., 0.));
            let y = spawn_at(&mut world, Vec3::new(0., -2000.1, 0.));
            let z = spawn_at(&mut world, Vec3::new(0., 0., -2000.1));

            world.run_system_once(when_going_outside_system).unwrap();

            let dead = died(&mut world);
            assert_eq!(dead.len(), 3);
            assert!(dead.contains(&x));
            assert!(dead.contains(&y));
            assert!(dead.contains(&z));
        }
    }

    /// `death_system`'s unit tests
    mod death_system {
        use super::super::death_system;
        use bevy::{ecs::system::RunSystemOnce, prelude::*};
        use spacerobo_commons::{DeathMessage, Hp, KillCounter};
        use spacerobo_target::Common as CommonTarget;

        /// A fresh world with `KillCounter`, an empty `DeathMessage` queue, and one dying
        /// entity (`Hp` plus, optionally, the `Target` marker) already reported dead.
        fn world_with_a_dying_entity(is_target: bool) -> (World, Entity) {
            let mut world = World::new();
            world.insert_resource(KillCounter::default());
            world.init_resource::<Messages<DeathMessage>>();

            let entity = if is_target {
                world.spawn((Hp::default(), CommonTarget)).id()
            } else {
                world.spawn(Hp::default()).id()
            };

            world
                .resource_mut::<Messages<DeathMessage>>()
                .write(DeathMessage::new(entity));

            (world, entity)
        }

        /// A target's death is counted.
        #[test]
        fn increments_the_kill_counter_when_a_target_dies() {
            let (mut world, _target) = world_with_a_dying_entity(true);

            world.run_system_once(death_system).unwrap();

            assert_eq!(**world.resource::<KillCounter>(), 1);
        }

        /// A non-target's death (e.g. the player leaving the world boundary) is not counted.
        #[test]
        fn does_not_increment_the_kill_counter_for_a_non_target_death() {
            let (mut world, _player) = world_with_a_dying_entity(false);

            world.run_system_once(death_system).unwrap();

            assert_eq!(**world.resource::<KillCounter>(), 0);
        }

        /// The dead entity is despawned regardless of whether it was a target.
        #[test]
        fn despawns_the_dead_entity() {
            let (mut world, target) = world_with_a_dying_entity(true);

            world.run_system_once(death_system).unwrap();

            assert!(world.get_entity(target).is_err());
        }
    }
}
