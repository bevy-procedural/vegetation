use bevy::{
    pbr::{MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    reflect::TypePath,
    render::{mesh::MeshVertexBufferLayout, render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}},
};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

#[derive(Component)]
pub struct MainCamera;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Settings {
    #[inspector(min = 0.0, max = 10.0)]
    pub box_size: f32,
    #[inspector(min = 0.0, max = 10.0)]
    pub box_thickness: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            box_size: 2.0,
            box_thickness: 0.15,
        }
    }
}

pub fn update_vegetation(_settings: Res<Settings>) {
    //println!("update")
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FernMaterial {
    #[uniform(100)]
    pub color: Color,
}

impl MaterialExtension for FernMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/fern.wgsl".into()
    }

    fn specialize(
        pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // disable backface culling
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
