// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use super::source::ImageExportSource;
use bevy::{
    ecs::query::QueryItem,
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::ExtractComponent, extract_resource::ExtractResource,
        render_asset::RenderAssets, render_resource::MapMode, renderer::RenderDevice,
        view::screenshot::ScreenshotManager, Extract, MainWorld,
    },
    utils::EntityHashMap,
};
use futures::channel::oneshot;
use wgpu::Maintain;

#[derive(Asset, Clone, Default, Reflect, Component)]
pub struct ImageExportSettings;

impl ExtractComponent for ImageExportSettings {
    type Query = (&'static Self, &'static Handle<ImageExportSource>);
    type Filter = ();
    type Out = (Self, Handle<ImageExportSource>);

    fn extract_component(
        (settings, source_handle): QueryItem<'_, Self::Query>,
    ) -> Option<Self::Out> {
        Some((settings.clone(), source_handle.clone_weak()))
    }
}

#[derive(Bundle, Default)]
pub struct ImageExportBundle {
    pub source: Handle<ImageExportSource>,
    pub settings: ImageExportSettings,
}

#[derive(Resource, Clone, Deref, ExtractResource, Reflect)]
pub struct ExtractableImage(pub Vec<u8>);

/*
pub struct ExtractedImage {
    pub data: Vec<u8>,
}

#[derive(Resource, Default)]
pub struct ExtractedImages {
    pub images: EntityHashMap<Entity, ExtractedImage>,
}*/

pub fn copy_from_render(
    mut commands: Commands,
    // export_bundles: Query<(&Handle<ImageExportSource>, &ImageExportSettings)>,
    // sources: Res<RenderAssets<ImageExportSource>>,
    //mut images: ResMut<Assets<Image>>,
    //mut extracted: ResMut<ExtractedImages>,
    //image_query: Extract<Query<(Entity, &Handle<Image>)>>,
    // extractable_image: Res<ExtractableImage>,
    
    //image_query: Extract<Query<(Entity, &Handle<Image>)>>,
    mut world: ResMut<MainWorld>,
    
) {
    //println!("Copying from render {}", extractable_image.0.len());
    // extracted.images.clear();

    //let images = world.get_resource::<ExtractedImages>().unwrap();
    //world.remove_resource::<ExtractedImages>().unwrap();

    /*for (entity, image) in image_query.iter() {
        println!("Image: {:?}", image);

    }*/
    /*extracted.images.clear();
    for (source_handle, settings) in &export_bundles {
        if let Some(gpu_source) = sources.get(source_handle) {
            let h = gpu_source.source_handle.clone().id();
            //let src_img = images.get_mut(h).unwrap();
            extracted.images.insert(
                h,
                ExtractedImage {
                    data: src_img.data.clone(),
                },
            );
        }
    }*/
}

pub fn store_in_img(
    export_bundles: Query<(&Handle<ImageExportSource>, &ImageExportSettings)>,
    sources: Res<RenderAssets<ImageExportSource>>,
    render_device: Res<RenderDevice>,
    mut extractable_image: ResMut<ExtractableImage>,
    mut gpu_images: ResMut<RenderAssets<Image>>,
) {
    for (source_handle, settings) in &export_bundles {
        if let Some(gpu_source) = sources.get(source_handle) {
            let mut image_bytes = {
                let slice = gpu_source.buffer.slice(..);
                {
                    let (mapping_tx, mapping_rx) = oneshot::channel();
                    render_device.map_buffer(&slice, MapMode::Read, move |res| {
                        mapping_tx.send(res).unwrap();
                    });
                    render_device.poll(Maintain::Wait);
                    futures_lite::future::block_on(mapping_rx).unwrap().unwrap();
                }
                slice.get_mapped_range().to_vec()
            };

            gpu_source.buffer.unmap();

            let bytes_per_row = gpu_source.bytes_per_row as usize;
            let padded_bytes_per_row = gpu_source.padded_bytes_per_row as usize;
            let source_size = gpu_source.source_size;
            if bytes_per_row != padded_bytes_per_row {
                let mut unpadded_bytes =
                    Vec::<u8>::with_capacity(source_size.height as usize * bytes_per_row);
                for padded_row in image_bytes.chunks(padded_bytes_per_row) {
                    unpadded_bytes.extend_from_slice(&padded_row[..bytes_per_row]);
                }
                image_bytes = unpadded_bytes;
            }

            //let h = gpu_source.source_handle.clone();
            // let src_img: &mut Image = images.get_mut(gpu_source.source_handle.clone()).unwrap();
            // src_img.data = image_bytes;

            //world.get_resource_mut::<FetchedImage>().unwrap().data = image_bytes;

            let gpu_image = gpu_images.get_mut(&gpu_source.source_handle).unwrap();

            println!(
                "Saving {}  {}",
                image_bytes.len(),
                extractable_image.0.len()
            );
            extractable_image.0 = image_bytes.clone();

            /*let mut writer = std::io::BufWriter::new(std::fs::File::create("test.png").unwrap());
            image::write_buffer_with_format(
                &mut writer,
                &image_bytes,
                gpu_image.size.y as u32,
                gpu_image.size.x as u32,
                image::ColorType::Rgba8,
                image::ImageFormat::Png,
            )
            .unwrap();*/

            //fetched.data = image_bytes;

            //println!("Saving {}", image_bytes.len());
        }
    }
}
