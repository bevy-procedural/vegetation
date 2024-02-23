// based on https://github.com/paulkre/bevy_image_export/blob/main/src/node.rs

use super::{fetch::ExtractableImage, source::ImageExportSource};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraphContext},
        render_resource::{ImageCopyBuffer, ImageDataLayout},
        renderer::RenderContext,
    },
};

pub const NODE_NAME: &str = "image_export";

pub struct ImageExportNode;
impl Node for ImageExportNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for (_, source) in world.resource::<RenderAssets<ImageExportSource>>().iter() {
            /*
            if world.get_resource::<ExtractableImage>().unwrap().0.len() > 0 {
                continue;
            }
            println!("Copying image data from GPU to CPU");
            */

            if let Some(gpu_image) = world
                .resource::<RenderAssets<Image>>()
                .get(&source.source_handle)
            {
                render_context.command_encoder().copy_texture_to_buffer(
                    gpu_image.texture.as_image_copy(),
                    ImageCopyBuffer {
                        buffer: &source.buffer,
                        layout: ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(source.padded_bytes_per_row),
                            rows_per_image: None,
                        },
                    },
                    source.source_size,
                );
            }
        }

        Ok(())
    }
}
