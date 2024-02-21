use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    pbr::{CascadeShadowConfigBuilder, ExtendedMaterial},
    prelude::*,
    render::mesh::PrimitiveTopology,
};
use bevy_editor_pls::prelude::*;
use std::{env, f32::consts::PI};

#[cfg(not(feature = "reload"))]
pub use components::*;
#[cfg(not(feature = "reload"))]
use procedural_vegetation::*;
#[cfg(feature = "reload")]
use procedural_vegetation_hot::*;
#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(
    dylib = "procedural_vegetation",
    file_watch_debounce = 200,
    lib_dir = "target/debug"
)]
mod procedural_vegetation_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}
}

fn reload_after_change() {
    #[cfg(feature = "reload")]
    if procedural_vegetation_hot::was_updated() {
        println!("Reloading systems");
    }
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    #[cfg(feature = "reload")]
    println!("Hello from the main module! This is a hot reloadable module.");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        /*.insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })*/
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, FernMaterial>,
        >::default())
        .register_type::<FernSettings>()
        .add_plugins((
            EditorPlugin::on_second_monitor_fullscreen(EditorPlugin::default()),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin::default(),
        ))
        .add_systems(Update, reload_after_change)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_vegetation)
        .add_systems(Update, update_vegetation)
        .run();
}

pub fn setup_vegetation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    mut materials2: ResMut<Assets<StandardMaterial>>,
    mut materials3: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    let count = 40 * 12;
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0., 0., 0.]].repeat(count));
    // TODO: to enable color in PBR (used for ao)
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1., 1., 1., 1.]].repeat(count));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]].repeat(count));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_TANGENT,
        vec![[0., 0., 0., 0.]].repeat(count),
    );

    let material = ExtendedMaterial::<StandardMaterial, FernMaterial> {
        base: StandardMaterial {
            //base_color: Color::rgb(0.1, 0.3, 0.1),
            base_color_texture: Some(render_texture(
                512,
                2048,
                &mut commands,
                &mut meshes,
                &mut materials3,
                &mut images,
                [
                    Color::rgb(0.1, 0.2, 0.0),
                    Color::rgb(0.0, 0.5, 0.0),
                    Color::rgb(0.0, 0.5, 0.0),
                ],
                1,
            )),
            normal_map_texture: Some(render_texture(
                512,
                2048,
                &mut commands,
                &mut meshes,
                &mut materials3,
                &mut images,
                [
                    Color::rgb(0.0, 0.0, 1.0),
                    Color::rgb(0.4, 0.0, 1.0),
                    Color::rgb(0.0, 0.0, 1.0),
                ],
                2,
            )),
            metallic: 0.4,
            perceptual_roughness: 0.2,
            reflectance: 0.0,
            double_sided: true,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        },
        extension: FernMaterial { time: 0.0 },
    };

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(material),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            subdivisions: 0,
            size: 8.0,
        })),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder::default())),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(-0.6, 0.7, 1.4),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    /*commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 3.0, 0.0),
        ..default()
    });*/

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 8.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
