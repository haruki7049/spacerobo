//! # Gun systems, components & etc...

pub mod bullet;
pub mod select_fire;

use self::select_fire::SelectFire;
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{DeathMessage, Hp, Weapon};
use spacerobo_target::Target;

/// Gun component
#[derive(Component)]
pub struct Gun {
    pub owner: Entity,

    /// Select fire setting
    pub select_fire: SelectFire,

    /// A interval settings and values
    pub interval: Interval,
}

impl Weapon for Gun {
    fn spawn_as_child(
        parent: &mut ChildSpawnerCommands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        origin: Vec3,
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
                        limit: 0.1,
                        rest: 0.0,
                        amount: 0.01,
                    },
                }),
                Collider::cylinder(0.125, 2.0),
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
                Transform::from_xyz(1., -1., -4.3).looking_to(Vec3::NEG_Z, Vec3::ZERO),
            ))
            // Muzzle
            .with_child((Transform::from_xyz(1., -1., -4.3), Muzzle));
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

pub fn gun_melee_damage_system(
    mut collision_event_reader: MessageReader<CollisionStart>,
    mut death_message_writer: MessageWriter<DeathMessage>,
    gun_query: Query<(), With<Gun>>,
    mut target_query: Query<&mut Hp, With<Target>>,
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

        const HUGE_DAMAGE: f32 = 20000.0;

        if let Ok(mut hp) = target_query.get_mut(target_entity) {
            // Decrease HP
            hp.rest -= HUGE_DAMAGE;

            if hp.rest <= 0. {
                death_message_writer.write(DeathMessage::new(target_entity));
            }
        }
    }
}
