use crate::target::Target;
use avian3d::prelude::*;
use bevy::prelude::*;

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
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    muzzle: Query<&GlobalTransform, With<Muzzle>>,
    gun: Query<&mut Gun>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    match gun.single().unwrap() {
        Gun {
            select_fire: SelectFire::Semi,
            ..
        } => semi_auto(commands, meshes, materials, muzzle, mouse),
        Gun {
            select_fire: SelectFire::Full,
            ..
        } => full_auto(commands, meshes, materials, gun, muzzle, mouse),
    }
}

/// Semi auto
fn semi_auto(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle: Query<&GlobalTransform, With<Muzzle>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        let global_transform = muzzle.single().unwrap();
        let bullet_origin: Vec3 = global_transform.translation();

        let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
        let bullet_force: Vec3 = direction * 200.0;
        debug!("bullet_force: {}", bullet_force);

        // ray_origin debugging by spawning a sphere
        commands.spawn((
            Transform::from_translation(bullet_origin),
            Mesh3d(meshes.add(Sphere::new(0.015625).mesh())),
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
    }
}

/// Full auto
fn full_auto(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gun: Query<&mut Gun>,
    muzzle: Query<&GlobalTransform, With<Muzzle>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        debug!("Mouse Left clicked");

        let mut gun = gun.single_mut().unwrap();
        if gun.interval.rest >= 0. {
            debug!("Full auto shoot aborted because of the gun's interval");
            return;
        }

        let global_transform = muzzle.single().unwrap();
        let bullet_origin: Vec3 = global_transform.translation();

        let direction: Vec3 = global_transform.rotation() * Vec3::NEG_Z;
        let bullet_force: Vec3 = direction * 200.0;
        debug!("bullet_force: {}", bullet_force);

        // ray_origin debugging by spawning a sphere
        commands.spawn((
            Transform::from_translation(bullet_origin),
            Mesh3d(meshes.add(Sphere::new(0.015625).mesh())),
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

        gun.interval.rest = gun.interval.limit;
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
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }

        if bullets.contains(*entity1) && targets.contains(*entity2) {
            commands.entity(*entity1).despawn();
            commands.entity(*entity2).despawn();
        }
    }
}
