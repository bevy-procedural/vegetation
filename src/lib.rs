use bevy::prelude::*;
use components::*;
use fern::{fern_mesh, FernPart};
pub mod fern;

#[no_mangle]
pub fn update_vegetation(
    query: Query<&FernSettings, Changed<FernSettings>>,
    mut assets: ResMut<Assets<Mesh>>,
) {
    for settings in query.iter() {
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
