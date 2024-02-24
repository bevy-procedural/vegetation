use bevy::prelude::*;
use components::*;
use fern::{fern_mesh, FernPart};
pub use setup::setup_vegetation;
pub mod fern;
mod setup;

#[no_mangle]
pub fn update_vegetation_off(query: Query<&FernSettings>, mut cameras: Query<&mut Camera>) {
    for settings in query.iter() {
        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            if !cam.is_active {
                continue;
            }
            println!("Deactivating camera");
            cam.is_active = false;
        }
    }
}

#[no_mangle]
pub fn update_vegetation(
    query: Query<&FernSettings, Changed<FernSettings>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut cameras: Query<&mut Camera>,
) {
    for settings in query.iter() {
        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            // TODO: this isn't always transferred in time...  How to make sure the camera is turned on in time?
            println!("Activating camera");
            cam.is_active = true;
        }

        //println!("Updating fern mesh");
        let fern = fern_mesh(settings, FernPart::Stem);
        let mesh = assets.get_mut(settings.meshes[0]).unwrap();
        fern.bevy_set(mesh);

        let fern = fern_mesh(settings, FernPart::LeafletTop);
        let mesh = assets.get_mut(settings.meshes[1]).unwrap();
        fern.bevy_set(mesh);

        let fern = fern_mesh(settings, FernPart::LeafletBottom);
        let mesh = assets.get_mut(settings.meshes[2]).unwrap();
        fern.bevy_set(mesh);
    }
}

#[no_mangle]
pub fn add_plugin(app: &mut App) {
    app.add_systems(Startup, setup_vegetation); //.add_systems(Startup, setup);
}
