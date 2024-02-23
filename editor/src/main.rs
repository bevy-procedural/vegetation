use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    pbr::ExtendedMaterial,
    prelude::*,
};
use bevy_editor_pls::prelude::*;
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

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    ));

    add_plugin(&mut app);

    app.add_systems(Update, reload_after_change)
        .add_systems(Update, bevy::window::close_on_esc)
        //.add_systems(Startup, setup_vegetation)
        .add_systems(Update, update_vegetation_off.before(update_vegetation))
        .add_systems(Update, update_vegetation)
        .run();
}
