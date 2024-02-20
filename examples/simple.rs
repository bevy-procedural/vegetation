use bevy::prelude::*;
use procedural_vegetation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Settings::default())
        .add_systems(Startup, setup_vegetation)
        .add_systems(Update, update_vegetation)
        .run();
}
