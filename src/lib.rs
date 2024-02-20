use bevy::{
    pbr::MaterialExtension, prelude::*, reflect::TypePath, render::{camera, render_resource::{AsBindGroup, ShaderRef}}
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

pub fn update_vegetation(settings: Res<Settings>) {
    //println!("update")
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FernMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,

    pub alpha_mode: AlphaMode,
}

impl MaterialExtension for FernMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fern.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/fern.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
