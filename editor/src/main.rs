use bevy::{pbr::wireframe::WireframeConfig, prelude::*, render::mesh::PrimitiveTopology};
use bevy_editor_pls::prelude::*;
use procedural_vegetation::*;
use std::env;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_plugins(MaterialPlugin::<FernMaterial>::default())
        .register_type::<Settings>()
        .insert_resource(Settings::default())
        .add_plugins(EditorPlugin::on_second_monitor_fullscreen(
            EditorPlugin::default(),
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_vegetation)
        .add_systems(Update, update_vegetation)
        .run();
}

pub fn setup_vegetation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FernMaterial>>,
    mut materials2: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0., 0., 0.]].repeat(40 * 12));

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(FernMaterial {
            color: Color::WHITE,
            color_texture: Some(asset_server.load("icon.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            subdivisions: 0,
            size: 1.0,
        })),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
