use crate::{player::Player, target::Target};
use avian3d::prelude::*;
use bevy::prelude::*;

const BULLET_SIZE: f32 = 1. / 16.;

/// Gun component
#[derive(Component, Default)]
pub struct Gun {
    /// Select fire setting
    pub select_fire: SelectFire,

    /// A interval settings and values
    pub interval: Interval,
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

/// Select fire setting for Gun component
#[derive(Clone, Copy, Default)]
pub enum SelectFire {
    /// Semi auto
    #[default]
    Semi,

    /// Full auto
    Full,
}

/// A marker component to know muzzle's transform
#[derive(Component)]
pub struct Muzzle;

/// A marker component for a bullet shot by a Gun
#[derive(Component)]
pub struct Bullet;

/// Gun shoot system
pub fn gun_shoot_system(
    commands: Commands,
    query: (
        Query<&mut Gun>,
        Query<&GlobalTransform, With<Muzzle>>,
        Query<&LinearVelocity, With<Player>>,
    ),
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    let (gun, _muzzle, _player) = &query;

    match gun.single().unwrap() {
        Gun {
            select_fire: SelectFire::Semi,
            ..
        } => semi_auto(commands, query, meshes, materials, mouse, asset_server),
        Gun {
            select_fire: SelectFire::Full,
            ..
        } => full_auto(commands, query, meshes, materials, mouse, asset_server),
    }
}

fn shoot(
    mut commands: Commands,
    query: (
        Query<&mut Gun>,
        Query<&GlobalTransform, With<Muzzle>>,
        Query<&LinearVelocity, With<Player>>,
    ),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let (_gun, muzzle, player) = query;

    let global_transform = muzzle.single().unwrap();
    let bullet_origin: Vec3 = global_transform.translation();
    let player_linear: Vec3 = player.single().unwrap().0;

    let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
    let bullet_force: Vec3 = direction * 200.0 + player_linear;
    debug!("bullet_force: {}", bullet_force);

    // ray_origin debugging by spawning a sphere
    commands.spawn((
        Transform::from_translation(bullet_origin),
        Mesh3d(meshes.add(Sphere::new(BULLET_SIZE).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        })),
        RigidBody::Dynamic,
        Collider::sphere(0.015625),
        LinearVelocity(bullet_force),
        CollisionEventsEnabled,
        Bullet,
    ));

    commands.spawn((
        Transform::from_translation(global_transform.translation()),
        AudioPlayer::new(asset_server.load("SE/shoot.ogg")),
        PlaybackSettings::ONCE.with_spatial(false),
    ));
}

/// Semi auto
fn semi_auto(
    commands: Commands,
    query: (
        Query<&mut Gun>,
        Query<&GlobalTransform, With<Muzzle>>,
        Query<&LinearVelocity, With<Player>>,
    ),
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        // Shoot!!
        shoot(commands, query, meshes, materials, asset_server);
    }
}

/// Full auto
fn full_auto(
    commands: Commands,
    mut query: (
        Query<&mut Gun>,
        Query<&GlobalTransform, With<Muzzle>>,
        Query<&LinearVelocity, With<Player>>,
    ),
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if mouse.pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        let (ref mut gun_query, _muzzle_query, _player_query) = query;
        let mut gun = gun_query.single_mut().unwrap();
        if gun.interval.rest >= 0. {
            debug!("Full auto shoot aborted because of the gun's interval");
            return;
        }

        // Full auto interval
        gun.interval.rest = gun.interval.limit;

        // Shoot!!
        shoot(commands, query, meshes, materials, asset_server);
    }
}

pub fn gun_cooling_system(mut gun: Query<&mut Gun>) {
    let mut gun = gun.single_mut().unwrap();

    gun.interval.rest -= gun.interval.amount;
}

pub fn toggle_select_fire_system(mut gun: Query<&mut Gun>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        let mut gun = gun.single_mut().unwrap();

        match gun.select_fire {
            SelectFire::Semi => gun.fullauto(),
            SelectFire::Full => gun.semiauto(),
        }
    }
}

pub fn bullet_hit_detection_system(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    targets: Query<Entity, With<Target>>,
    bullets: Query<Entity, With<Bullet>>,
    asset_server: Res<AssetServer>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!("Collision!!");

        if targets.contains(*entity1) && targets.contains(*entity2) {
            return;
        }

        if bullets.contains(*entity1) && bullets.contains(*entity2) {
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if targets.contains(*entity1) && bullets.contains(*entity2) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/kill.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if bullets.contains(*entity1) && targets.contains(*entity2) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/kill.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }
    }
}
