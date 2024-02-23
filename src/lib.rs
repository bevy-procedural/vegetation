use bevy::{prelude::*, render::renderer::RenderDevice};
use components::*;
use fern::{fern_mesh, FernPart};
use setup::setup_vegetation;
mod fern;
mod setup;

#[no_mangle]
pub fn fetch(world: &World, mut images: ResMut<Assets<Image>>) {
    let device = world.resource::<RenderDevice>();
    let ctx = bevy::render::renderer::RenderContext::new(device.clone());

    /* for settings in world.query::<&FernSettings>().iter(world) {
        let image = images.get(settings.render_target.clone().unwrap()).unwrap();

        /*ctx.command_encoder()
        .copy_texture_to_buffer(image, destination, copy_size);*/
    }*/
}

#[no_mangle]
pub fn update_vegetation_off(query: Query<&FernSettings>, mut cameras: Query<&mut Camera>) {
    for settings in query.iter() {
        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            if !cam.is_active {
                continue;
            }
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
