use bevy::{
    pbr::{CascadeShadowConfigBuilder, ExtendedMaterial},
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::{NoFrustumCulling, RenderLayers},
    },
};
use components::*;
use std::f32::consts::PI;

pub fn render_to_texture(
    width: u32,
    height: u32,
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    layer: u8,
) -> (Handle<Image>, RenderLayers, Entity) {
    let size = Extent3d {
        width,
        height,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(layer);

    let camera_id = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d { ..default() },
                camera: Camera {
                    // render before the "main pass" camera
                    order: -1,
                    clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                    target: image_handle.clone().into(),
                    ..default()
                },
                ..default()
            },
            first_pass_layer,
        ))
        .id();

    return (image_handle, first_pass_layer, camera_id);
}

pub fn render_texture(
    width: u32,
    height: u32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    colors: [Color; 3],
    layer: u8,
) -> Handle<Image> {
    let mut settings = FernSettings {
        width,
        height,
        ..default()
    };

    let (img, layer, camera_id) = render_to_texture(width, height, commands, images, layer);
    let mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let mesh2 = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let mesh3 = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    settings.meshes = vec![mesh.id(), mesh2.id(), mesh3.id()];
    settings.camera = Some(camera_id);
    settings.render_target = Some(img.clone());

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

    return img;
}

pub fn setup_vegetation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    mut materials3: ResMut<Assets<ColorMaterial>>,
    mut materials2: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip, RenderAssetUsages::all());
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
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(0.0, 1.0, 0.0)))),
        material: materials2.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.4),
            ..default()
        }),
        transform: Transform::from_scale(Vec3::splat(100.0)),
        ..default()
    },));

    
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
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
