pub mod health;

use crate::{DeathEvent, GameMode, Hp};
use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
};

#[derive(Component)]
pub struct Target;

pub fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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

    let targets_translation: Vec<(Vec3, Color)> = vec![
        (Vec3::new(10.0, 0.0, 0.0), RED.into()),
        (Vec3::new(0.0, 10.0, 0.0), BLUE.into()),
        (Vec3::new(0.0, 0.0, 10.0), GREEN.into()),
        (Vec3::new(-10.0, 0.0, 0.0), YELLOW.into()),
        (Vec3::new(0.0, -10.0, 0.0), SILVER.into()),
        (Vec3::new(0.0, 0.0, -10.0), BLACK.into()),
    ];

    // Targets
    for (translation, base_color) in targets_translation {
        commands.spawn((
            StateScoped(GameMode::ShootingRange),
            Mesh3d(meshes.add(Sphere::default().mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color,
                ..Default::default()
            })),
            Transform::from_translation(translation),
            RigidBody::Static,
            Collider::sphere(1.0),
            CollisionEventsEnabled,
            Mass(1.0),
            Target,
            Hp::default(),
        ));
    }
}

/// This system detects the hits between two objects, having Hp, LinearVelocity and Mass Components.
/// This system is created to decrease the hp at contacted objects.
pub fn collision_detection_system(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut query: Query<(&mut Hp, &LinearVelocity, &Mass)>,
    mut event_writer: EventWriter<DeathEvent>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        let objects = query.get_many_mut([*entity1, *entity2]).ok();

        match objects {
            Some([mut obj1, mut obj2]) => {
                let obj1_damage: f32 = calc_damage(&obj1);
                let obj2_damage: f32 = calc_damage(&obj2);
                let damage: f32 = obj1_damage + obj2_damage;

                let (ref mut obj1_hp, _obj1_linear, _obj1_mass) = obj1;
                let (ref mut obj2_hp, _obj2_linear, _obj2_mass) = obj2;

                obj1_hp.rest -= damage;
                obj2_hp.rest -= damage;

                debug!("The first object's Hp: {:?}", &obj1_hp);
                debug!("The second object's Hp: {:?}", &obj2_hp);

                if obj1_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(*entity1));
                }
                if obj2_hp.rest <= 0. {
                    event_writer.write(DeathEvent::new(*entity2));
                }
            }
            _ => debug!(
                "The collisioned entity, {} or {} is missing Hp, LinearVelocity or Mass",
                entity1, entity2
            ),
        }
    }
}

fn calc_damage(object: &(Mut<'_, Hp>, &LinearVelocity, &Mass)) -> f32 {
    let (_hp, linear, mass) = object;

    let speed: f32 = linear.x + linear.y + linear.z;

    // Speed * Mass = Force
    // By Isaac Newton
    // Probably...
    (speed * ***mass).abs()
}
