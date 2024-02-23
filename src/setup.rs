use bevy::{
    pbr::{CascadeShadowConfigBuilder, ExtendedMaterial},
    prelude::*,
    render::{
        mesh::shape::Cube,
        renderer::{RenderContext, RenderDevice},
        texture::{CompressedImageFormats, ImageSampler, ImageType},
        view::NoFrustumCulling,
    },
};
use components::*;
pub use super::gpu2cpu::{
    fetch::{ImageExportBundle, ImageExportPlugin},
    source::ImageExportSource,
};
use procedural_meshes::{fill::MyFill, mesh::MyMesh, *};
use std::f32::consts::PI;
use wgpu::PrimitiveTopology;

pub fn render_texture(
    width: u32,
    height: u32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    colors: [Color; 3],
    layer: u8,
    _device: Res<bevy::render::renderer::RenderDevice>,
    export_sources: &mut ResMut<Assets<ImageExportSource>>,
) -> Handle<Image> {
    let mut settings = FernSettings {
        width,
        height,
        ..default()
    };
    //let fern = fern_mesh(&settings);
    //let fern_mesh = fern.to_bevy();

    let (img, layer, camera_id) = render_to_texture(width, height, commands, images, layer);
    let mesh = meshes.add(Mesh::from(Cube { size: 1.0 }));
    let mesh2 = meshes.add(Mesh::from(Cube { size: 1.0 }));
    let mesh3 = meshes.add(Mesh::from(Cube { size: 1.0 }));
    settings.meshes = vec![mesh.id(), mesh2.id(), mesh3.id()];
    settings.camera = Some(camera_id);
    settings.render_target = Some(img.clone());

    /*
    let supported_compressed_formats = CompressedImageFormats::from_features(device.features());
    let image = images.get(img.clone()).unwrap();
    let compressed_basis_data = compress_to_basis(&image);
    let comp_img = Image::from_buffer(
        &compressed_basis_data,
        ImageType::Format(bevy::render::texture::ImageFormat::Basis),
        supported_compressed_formats,
        image.texture_descriptor.format.is_srgb(),
        ImageSampler::linear(),
    )
    .unwrap();
    let image_handle = images.add(comp_img);
    settings.compressed_target = Some(image_handle.clone());*/

    commands
        .spawn((
            ColorMesh2dBundle {
                mesh: mesh.into(),
                material: materials.add(ColorMaterial::from(colors[0])),
                ..default()
            },
            layer,
            Name::new("fern"),
            settings,
        ))
        .with_children(|parent| {
            parent.spawn((
                ColorMesh2dBundle {
                    mesh: mesh2.clone().into(),
                    material: materials.add(ColorMaterial::from(colors[1])),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                    ..default()
                },
                layer,
            ));
            parent.spawn((
                ColorMesh2dBundle {
                    mesh: mesh3.into(),
                    material: materials.add(ColorMaterial::from(colors[2])),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                    ..default()
                },
                layer,
            ));
        });

    commands.spawn(ImageExportBundle {
        source: export_sources.add(img.clone().into()),
        ..default()
    });

    return img;
}

pub fn setup_vegetation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    mut materials2: ResMut<Assets<StandardMaterial>>,
    mut materials3: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    device: Res<RenderDevice>,
    //asset_server: Res<AssetServer>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    let count = 40 * 12;
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0., 0., 0.]].repeat(count));
    // TODO: to enable color in PBR (used for ao). Is there a way without adding an attribute?
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1., 1., 1., 1.]].repeat(count));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]].repeat(count));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_TANGENT,
        vec![[0., 0., 0., 0.]].repeat(count),
    );

    // Some(asset_server.load("test.basis"));
    let fern = Some(render_texture(
        512,
        2048,
        &mut commands,
        &mut meshes,
        &mut materials3,
        &mut images,
        [
            Color::rgb(0.1, 0.2, 0.0),
            Color::rgb(0.05, 0.3, 0.0),
            Color::rgb(0.05, 0.36, 0.05),
        ],
        1,
        device,
        &mut export_sources,
    ));

    let fern_normal = None;
    /*Some(render_texture(
        512,
        2048,
        &mut commands,
        &mut meshes,
        &mut materials3,
        &mut images,
        [
            Color::rgb(0.0, 0.0, 1.0),
            Color::rgb(0.4, 0.0, 1.0),
            Color::rgb(0.0, 0.0, 1.0),
        ],
        2,device
    ));*/

    let material = ExtendedMaterial::<StandardMaterial, FernMaterial> {
        base: StandardMaterial {
            base_color_texture: fern,
            normal_map_texture: fern_normal,
            metallic: 0.4,
            perceptual_roughness: 0.2,
            reflectance: 0.0,
            double_sided: true,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        },
        extension: FernMaterial { time: 0.0 },
    };

    // TODO: use instancing https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/shader/shader_instancing.rs#L104

    for i in 0..30 {
        let s = (i as f32 * 100.0).sin() + 2.0;

        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(mesh.clone()),
                transform: Transform::from_xyz(
                    ((1012.0 * i as f32).sin() * 100000.0) % 10.0,
                    s / 2.0,
                    ((432.0 * i as f32).sin() * 100000.0) % 10.0,
                )
                .with_scale(Vec3::splat(s)),
                material: materials.add(material.clone()),
                ..default()
            },
            // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
            // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
            // instanced cubes will be culled.
            // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
            // instancing, and that is not taken into account with the built-in frustum culling.
            // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
            // component to avoid incorrect culling.
            NoFrustumCulling,
        ));
    }

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            subdivisions: 0,
            size: 100.0,
        })),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder::default())),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        }),
        transform: Transform::from_xyz(-0.6, 0.7, 1.4),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    /*commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 3.0, 0.0),
        ..default()
    });*/

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        // High-Quality Shadows!
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.001,
            maximum_distance: 30.0,
            first_cascade_far_bound: 10.0,
            overlap_proportion: 0.2,

            ..default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
