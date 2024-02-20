use bevy::prelude::*;
use procedural_vegetation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update_vegetation)
        .run();
}
