// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use super::source::ImageExportSource;
use crate::gpu2cpu::node::{ImageExportNode, NODE_NAME};
use bevy::{
    ecs::query::QueryItem,
    prelude::*,
    render::{
        camera::CameraUpdateSystem,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        main_graph::node::CAMERA_DRIVER,
        render_asset::{RenderAssetPlugin, RenderAssets},
        render_graph::RenderGraph,
        render_resource::MapMode,
        renderer::RenderDevice,
        Render, RenderApp, RenderSet,
    },
};
use futures::channel::oneshot;
use wgpu::Maintain;

#[derive(Asset, Clone, Default, Reflect, Component)]
pub struct ImageExportSettings {}

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

fn save_buffer_to_disk(
    export_bundles: Query<(&Handle<ImageExportSource>, &ImageExportSettings)>,
    sources: Res<RenderAssets<ImageExportSource>>,
    render_device: Res<RenderDevice>,
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

            // let settings = settings.clone();
            //let frame_id = *frame_id - start_frame.0 + 1;
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

            println!("Saving {}", image_bytes.len());

            /*
            export_threads.count.fetch_add(1, Ordering::SeqCst);
            std::thread::spawn(move || {
                if bytes_per_row != padded_bytes_per_row {
                    let mut unpadded_bytes =
                        Vec::<u8>::with_capacity(source_size.height as usize * bytes_per_row);

                    for padded_row in image_bytes.chunks(padded_bytes_per_row) {
                        unpadded_bytes.extend_from_slice(&padded_row[..bytes_per_row]);
                    }

                    image_bytes = unpadded_bytes;
                }

                println!("Saving {}", image_bytes.len());

                /*fn save_buffer<P: Pixel + PixelWithColorType>(
                    image_bytes: &[P::Subpixel],
                    source_size: &Extent3d,
                    path: &str,
                ) where
                    P::Subpixel: AnyBitPattern,
                    [P::Subpixel]: EncodableLayout,
                {
                    match ImageBuffer::<P, _>::from_raw(
                        source_size.width,
                        source_size.height,
                        image_bytes,
                    ) {
                        Some(buffer) => match buffer.save(path) {
                            Err(ImageError::Unsupported(err)) => {
                                if let UnsupportedErrorKind::Format(hint) = err.kind() {
                                    println!("Image format {} is not supported", hint);
                                }
                            }
                            _ => {}
                        },
                        None => {
                            println!("Failed creating image buffer for '{}'", path);
                        }
                    }
                }

                match settings.extension.as_str() {
                    "exr" => {
                        save_buffer::<Rgba<f32>>(
                            bytemuck::cast_slice(&image_bytes),
                            &source_size,
                            path.as_str(),
                        );
                    }
                    _ => {
                        save_buffer::<Rgba<u8>>(&image_bytes, &source_size, path.as_str());
                    }
                }*/

                export_threads.count.fetch_sub(1, Ordering::SeqCst);
            });*/
        }
    }
}

/// Plugin enabling the generation of image sequences.
#[derive(Default)]
pub struct ImageExportPlugin {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ImageExportSystems {
    SetupImageExport,
    SetupImageExportFlush,
}

impl Plugin for ImageExportPlugin {
    fn build(&self, app: &mut App) {
        use ImageExportSystems::*;

        app.configure_sets(
            PostUpdate,
            (SetupImageExport, SetupImageExportFlush)
                .chain()
                .before(CameraUpdateSystem),
        )
        .register_type::<ImageExportSource>()
        .init_asset::<ImageExportSource>()
        .register_asset_reflect::<ImageExportSource>()
        .add_plugins((
            RenderAssetPlugin::<ImageExportSource>::default(),
            ExtractComponentPlugin::<ImageExportSettings>::default(),
        ))
        .add_systems(PostUpdate, apply_deferred.in_set(SetupImageExportFlush));

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(
            Render,
            save_buffer_to_disk
                .after(RenderSet::Render)
                .before(RenderSet::Cleanup),
        );

        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

        graph.add_node(NODE_NAME, ImageExportNode);
        graph.add_node_edge(CAMERA_DRIVER, NODE_NAME);
    }
}
