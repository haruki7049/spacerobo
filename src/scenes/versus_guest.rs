mod client;
mod entities;
mod health;

use crate::{DeathEvent, GameMode, Hp, KillCounter, OpponentResource};
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct VersusGuestPlugin;

impl Plugin for VersusGuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>();
        app.insert_resource(Gravity(Vec3::NEG_Y * 0.));
        app.insert_resource(KillCounter::default());
        app.insert_resource(OpponentResource::default());
        app.add_systems(
            OnEnter(GameMode::VersusGuest),
            (
                setup_system,
                entities::player::setup_system,
                entities::player::ui::setup_system,
                client::setup_system,
            ),
        );
        app.add_systems(
            Update,
            (
                // Player
                entities::player::respawn_system,
                entities::player::ui::update_system,
                entities::player::gun::select_fire::full_auto_system,
                entities::player::gun::select_fire::semi_auto_system,
                entities::player::gun::select_fire::toggle_select_fire_system,
                entities::player::gun::select_fire::timer_system,
                entities::player::gun::bullet::health::update_system,
                entities::player::health::update_system,
                // Opponent
                entities::opponent::update_system,
                entities::opponent::health::update_system,
                entities::opponent::bullet::update_system,
                // Systems
                health::update_system,
                client::update_system,
                collision_detection_system,
            )
                .run_if(in_state(GameMode::VersusGuest)),
        );
        app.add_systems(
            FixedUpdate,
            (
                // Player movement systems
                entities::player::movement::keyboard::update_system,
                entities::player::movement::mouse::update_system,
                entities::player::movement::controller::update_system,
                // Player gun systems
                entities::player::gun::gun_cooling_system,
            )
                .run_if(in_state(GameMode::VersusGuest)),
        );
        app.add_observer(client::on_connecting);
        app.add_observer(client::on_connected);
        app.add_observer(client::on_disconnected);
    }
}

#[derive(Component)]
struct Target;

fn setup_system(mut commands: Commands) {
    // Light
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 2.0),
    ));
}

/// This system detects the hits between two objects, having Hp, LinearVelocity and Mass Components.
/// This system is created to decrease the hp at contacted objects.
fn collision_detection_system(
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
