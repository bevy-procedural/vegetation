use bevy::{prelude::*, render::renderer::RenderDevice};
use components::*;
pub use gpu2cpu::{
    fetch::{ImageExportBundle, ImageExportPlugin},
    source::ImageExportSource,
};
use setup::{fern_mesh, setup_vegetation, FernPart};
mod compress;
pub mod gpu2cpu;
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
pub fn update_vegetation_off(
    query: Query<&FernSettings>,
    mut cameras: Query<&mut Camera>,
    mut images: ResMut<Assets<Image>>,
    //mut materials: ResMut<Assets<bevy::pbr::ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    //device: Res<bevy::render::renderer::RenderDevice>,
) {
    for settings in query.iter() {
        let image = images.get(settings.render_target.clone().unwrap()).unwrap();
        image
            .clone()
            .try_into_dynamic()
            .unwrap()
            .to_rgba8()
            .save("test.png")
            .unwrap();

        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            if !cam.is_active {
                continue;
            }
            cam.is_active = false;

            /*println!("Compressing image");
            let is_srgb;
            let compressed_basis_data = {
                let image = images.get(settings.render_target.clone().unwrap()).unwrap();
                is_srgb = image.texture_descriptor.format.is_srgb();
                compress_to_basis(&image)
            };
            /*let target = images
            .get_mut(settings.compressed_target.clone().unwrap())
            .unwrap();*/
            let supported_compressed_formats =
                CompressedImageFormats::from_features(device.features());
            let comp_img = Image::from_buffer(
                &compressed_basis_data,
                ImageType::Format(bevy::render::texture::ImageFormat::Basis),
                supported_compressed_formats,
                is_srgb,
                ImageSampler::linear(),
            )
            .unwrap();
            //target.data = comp_img.data;

            // TODO: leaks memory

            let image = images.get(settings.render_target.clone().unwrap()).unwrap();

            image
                .clone()
                .try_into_dynamic()
                .unwrap()
                .save("test.png")
                .unwrap();

            /*let mut writer = std::io::BufWriter::new(std::fs::File::create("test.png").unwrap());
            image::write_buffer_with_format(
                &mut writer,
                &image.data,
                image.width(),
                image.height(),
                image::ColorType::Rgba8,
                image::ImageFormat::Png,
            )
            .unwrap();*/

            /*let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open("test.basis")
                .unwrap();
            use std::io::Write;
            file.write_all(&compressed_basis_data).unwrap();*/

            let new_handle = images.add(comp_img);

            for m in materials.iter_mut() {
                let x = m.1;
                x.base.base_color_texture = Some(settings.render_target.clone().unwrap());
                x.base.base_color_texture = Some(new_handle.clone());
            }*/
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
    app.add_plugins(ImageExportPlugin::default())
        .add_systems(Startup, setup_vegetation); //.add_systems(Startup, setup);
}
