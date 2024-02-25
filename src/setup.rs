use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages, view::RenderLayers},
};
use components::*;
use render_to_texture::create_render_texture;

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

    let (img, _) = create_render_texture(width, height, commands, images, layer, true);
    let layer = RenderLayers::layer(layer);
    let mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let mesh2 = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let mesh3 = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    settings.meshes = vec![mesh.id(), mesh2.id(), mesh3.id()];
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

pub fn make_fern_mesh() -> Mesh {
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
    return mesh;
}

pub fn make_fern_material(
    fern_color: Option<Handle<Image>>,
    fern_normal: Option<Handle<Image>>,
) -> bevy::pbr::ExtendedMaterial<StandardMaterial, FernMaterial> {
    let material = bevy::pbr::ExtendedMaterial::<StandardMaterial, FernMaterial> {
        base: StandardMaterial {
            base_color_texture: fern_color,
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

    return material;
}
