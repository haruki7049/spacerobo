//! # Target systems, Compoments & etc...

use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{GameMode, Hp};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_target_assets);
    }
}

const TARGET_MASS: f32 = 1.0;
const TARGET_SIZE: f32 = 1.0;

/// Target Component
#[derive(Component)]
#[require(
    RigidBody::Static,
    Collider::sphere(TARGET_SIZE),
    CollisionEventsEnabled,
    Mass(TARGET_MASS)
)]
pub struct Target {
    pub base_color: Color,
}

impl Target {
    pub fn spawn(commands: &mut Commands, position: Vec3, base_color: Color) {
        commands.spawn((
            DespawnOnExit(GameMode::InGame),
            Self { base_color },
            Transform::from_translation(position),
        ));
    }
}

pub fn attach_target_assets(
    on: On<Add, Target>,
    mut commands: Commands,
    query: Query<&Target>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(target) = query.get(on.entity) else {
        return;
    };

    commands.entity(on.entity).insert((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: target.base_color,
            ..Default::default()
        })),
        Hp::robo(Some(asset_server.load("SE/kill.ogg"))),
    ));
}

#[cfg(test)]
mod tests {
    use crate::{Target, attach_target_assets};
    use avian3d::prelude::*;
    use bevy::{
        color::palettes::css::RED, mesh::MeshPlugin, prelude::*,
        render::render_asset::RenderAssetPlugin,
    };
    use spacerobo_commons::Hp;

    #[test]
    fn spawn_target() {
        let mut app = App::new();
        app.add_plugins((MeshPlugin::default(), AssetPlugin::default()));
        app.add_observer(attach_target_assets);

        let target_id: Entity = app
            .world_mut()
            .spawn(Target {
                base_color: RED.into(),
            })
            .id();

        app.update();
        assert!(app.world().get::<Target>(target_id).is_some());
        assert!(app.world().get::<Hp>(target_id).is_some());
        assert!(app.world().get::<RigidBody>(target_id).is_some());
        assert!(app.world().get::<Collider>(target_id).is_some());
        assert!(app.world().get::<Mass>(target_id).is_some());
        assert!(app.world().get::<Mesh3d>(target_id).is_some());
        assert!(
            app.world()
                .get::<MeshMaterial3d<StandardMaterial>>(target_id)
                .is_some()
        );
        assert!(
            app.world()
                .get::<CollisionEventsEnabled>(target_id)
                .is_some()
        );
    }
}
