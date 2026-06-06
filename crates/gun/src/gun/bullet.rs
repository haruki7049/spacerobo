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
            Collider::sphere(0.015625),
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

// Bullet specific collision system
pub fn bullet_collision_system(
    mut commands: Commands,
    mut collision_event_reader: MessageReader<CollisionStart>,
    mut bullet_query: Query<(&mut Common, &LinearVelocity, &Mass)>,
    other_query: Query<(Option<&LinearVelocity>, Option<&Mass>)>,
) {
    for event in collision_event_reader.read() {
        let e1 = event.collider1;
        let e2 = event.collider2;

        let mut process_collision = |bullet_entity, other_entity| {
            if let Ok((mut bullet, b_vel, b_mass)) = bullet_query.get_mut(bullet_entity) {
                // Ignore owner collision if bounce_count is 0
                if bullet.bounce_count == 0 && bullet.owner == other_entity {
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
                bullet.bounce_count += 1;
            }
        };

        process_collision(e1, e2);
        process_collision(e2, e1);
    }
}
