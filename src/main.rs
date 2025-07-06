use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::{BLACK, BLUE, GREEN, RED, SILVER, YELLOW},
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use spacerobo::player;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, author, about)]
struct CLIArgs {
    config: Option<PathBuf>,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct GameSettings {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let settings_path: PathBuf = args.config.unwrap_or_else(|| {
        confy::get_configuration_file_path("spacerobo", "config")
            .expect("Failed to get path for spacerobo")
    });
    let settings: GameSettings = confy::load_path(&settings_path)?;

    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec3::NEG_Y * 0.))
        .insert_resource(settings)
        .add_systems(Startup, setup)
        .add_systems(Startup, player::setup)
        .add_systems(
            Update,
            (
                player::keyboard_mouse_system,
                player::controller_system,
                player::ui_system,
            ),
        )
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light
    commands.spawn((
        EnvironmentMapLight {
            diffuse_map: images.add(uv_debug_texture()),
            specular_map: images.add(uv_debug_texture()),
            intensity: 5.0,
            rotation: Quat::NAN,
            affects_lightmapped_mesh_diffuse: false,
        },
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 2.0),
    ));

    // stones to distinguish directories
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Transform::from_xyz(10.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        Transform::from_xyz(-10.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: YELLOW.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, -10.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SILVER.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
