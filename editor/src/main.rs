use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, pbr::ExtendedMaterial, prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use std::env;

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
    .add_plugins(MaterialPlugin::<
        ExtendedMaterial<StandardMaterial, FernMaterial>,
    >::default())
    .register_type::<FernSettings>()
    .add_plugins((
        FrameTimeDiagnosticsPlugin,
        FilterQueryInspectorPlugin::<With<FernSettings>>::default(),
    ));

    add_plugin(&mut app);

    #[cfg(feature = "reload")]
    app.add_systems(PreUpdate, reload_after_change);

    app.add_systems(
        Update,
        (
            update_vegetation_off,
            update_vegetation.after(update_vegetation_off),
            bevy::window::close_on_esc,
        ),
    )
    .run();
}
