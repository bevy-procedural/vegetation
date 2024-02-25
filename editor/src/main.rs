use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::{CascadeShadowConfigBuilder, ExtendedMaterial},
    prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_panorbit_camera::*;
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

#[cfg(feature = "reload")]
fn reload_after_change(mut query: Query<&mut FernSettings>) {
    if procedural_vegetation_hot::was_updated() {
        println!("Reloading systems");
        for mut settings in query.iter_mut() {
            settings.version = settings.version + 1;
        }
    }
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    #[cfg(feature = "reload")]
    println!("Hello from the main module! This is a hot reloadable module.");

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1920.0, 1080.0),
            position: WindowPosition::Centered(MonitorSelection::Index(1)),
            decorations: false,
            ..default()
        }),
        ..default()
    }))
    /*.insert_resource(WireframeConfig {
        global: true,
        default_color: Color::WHITE,
    })*/
    .add_plugins((
        MaterialPlugin::<ExtendedMaterial<StandardMaterial, FernMaterial>>::default(),
        VegetationPlugin,
    ))
    .register_type::<FernSettings>()
    .add_systems(Startup, setup_scene)
    .add_plugins((
        FrameTimeDiagnosticsPlugin,
        //LogDiagnosticsPlugin::default(),
        FilterQueryInspectorPlugin::<With<FernSettings>>::default(),
        PanOrbitCameraPlugin,
    ));

    #[cfg(feature = "reload")]
    app.add_systems(PreUpdate, reload_after_change);

    app.add_systems(Update, (update_vegetation, bevy::window::close_on_esc))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // TODO: use instancing https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/shader/shader_instancing.rs#L104

    render_texture(
        2048,
        512,
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &mut images,
        [
            Color::rgb(0.1, 0.2, 0.0),
            Color::rgb(0.05, 0.3, 0.0),
            Color::rgb(0.05, 0.36, 0.05),
        ],
        1,
    );

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0)))),
        material: standard_materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.4),
            ..default()
        }),
        transform: Transform::from_scale(Vec3::splat(100.0)),
        ..default()
    },));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
        material: standard_materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(-0.6, 0.7, 1.4).with_scale(Vec3::new(1.0, 2.0, 1.0)),
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
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        // High-Quality Shadows!
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.001,
            maximum_distance: 30.0,
            first_cascade_far_bound: 10.0,
            overlap_proportion: 0.2,

            ..default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
