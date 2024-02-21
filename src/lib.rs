use bevy::{
    prelude::*,
    render::{
        mesh::shape::Cube,
        texture::{CompressedImageFormats, ImageSampler, ImageType},
    },
};
use components::*;
use procedural_meshes::{fill::MyFill, mesh::MyMesh, *};

fn compress_to_basis(image: &Image) -> Vec<u8> {
    // from bevy::render::texture::CompressedImageSaver:

    // PERF: this should live inside the future, but CompressorParams and Compressor are not Send / can't be owned by the BoxedFuture (which _is_ Send)
    let mut compressor_params = basis_universal::CompressorParams::new();
    compressor_params.set_basis_format(basis_universal::BasisTextureFormat::UASTC4x4);
    compressor_params.set_generate_mipmaps(true);
    let is_srgb = image.texture_descriptor.format.is_srgb();
    let color_space = if is_srgb {
        basis_universal::ColorSpace::Srgb
    } else {
        basis_universal::ColorSpace::Linear
    };
    compressor_params.set_color_space(color_space);
    compressor_params.set_uastc_quality_level(basis_universal::UASTC_QUALITY_DEFAULT);
    compressor_params.set_etc1s_quality_level(basis_universal::ETC1S_QUALITY_DEFAULT);

    let mut source_image = compressor_params.source_image_mut(0);
    let size = image.size();
    source_image.init(&image.data, size.x, size.y, 4);

    let mut compressor = basis_universal::Compressor::new(4);
    // SAFETY: the CompressorParams are "valid" to the best of our knowledge. The basis-universal
    // library bindings note that invalid params might produce undefined behavior.
    unsafe {
        compressor.init(&compressor_params);
        compressor.process().unwrap();
    }
    let compressed_basis_data = compressor.basis_file().to_vec();

    //println!("Original data size: {}", image.data.len());
    //println!("Compressed data size: {}", compressed_basis_data.len());

    return compressed_basis_data;
}

#[no_mangle]
pub fn update_vegetation_off(
    query: Query<&FernSettings>,
    mut cameras: Query<&mut Camera>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<bevy::pbr::ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    device: Res<bevy::render::renderer::RenderDevice>,
) {
    for settings in query.iter() {
        if let Ok(mut cam) = cameras.get_mut(settings.camera.unwrap()) {
            if !cam.is_active {
                continue;
            }
            cam.is_active = false;

            /*
            println!("Compressing image");
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

#[derive(Debug, Reflect, Component, PartialEq)]
enum FernPart {
    Stem,
    LeafletTop,
    LeafletBottom,
}

fn fern_mesh(settings: &FernSettings, part: FernPart) -> MyMesh {
    let mut fill = MyFill::new(0.0001);
    fill.draw(|builder| {
        let stem_w = settings.stem_w;
        let stem_w2 = settings.stem_w2;

        if part == FernPart::Stem {
            builder.begin(Vec2::new(0.0, stem_w));
            builder.line_to(Vec2::new(1.0, stem_w2));
            builder.line_to(Vec2::new(1.0, -stem_w2));
            builder.line_to(Vec2::new(0.0, -stem_w));
            builder.end(true);
        }

        fn leaflet(
            start: Vec2,
            leaflets: u32,
            leaflet_len: f32,
            curve: f32,
            l0: f32,
            dir: f32,
            builder: &mut builder::Builder,
            settings: &FernSettings,
            part: &FernPart,
        ) {
            builder.push();
            let c0 = curve;
            let a0 = leaflet_len / ((leaflets + 1) as f32 * 0.5);
            builder.translate(start);
            for i in 0..(leaflets - 2) {
                let prog = 1.0 - i as f32 / leaflets as f32;
                let l = l0 * prog * leaflet_len;
                let a = dir * a0 * prog;
                let step = Vec2::new(c0, a);
                //builder.rotate(-curve * 2.0 * dir); // TODO: rotation can be better controlled. However, I like the current ones since they have more imperfections
                let slant = settings.slant;
                let thinning = settings.thinning;
                let stomp = settings.stomp;

                if *part == FernPart::LeafletTop {
                    builder
                        .begin_here()
                        .quadratic_bezier_to(
                            Vec2::new(l * stomp, thinning * a * (-0.5 + slant)),
                            Vec2::new(l, thinning * a * (0.5 + slant)),
                        )
                        .quadratic_bezier_to(Vec2::new(l, thinning * a * (1.0 + slant)), step)
                        .close();
                }

                if *part == FernPart::LeafletBottom {
                    let l2 = -(l - 2.0 * c0);
                    builder
                        .begin_here()
                        .quadratic_bezier_to(
                            Vec2::new(l2 * stomp, thinning * a * (-0.5 + slant)),
                            Vec2::new(l2, thinning * a * (0.5 + slant)),
                        )
                        .quadratic_bezier_to(Vec2::new(l2, thinning * a * (1.0 + slant)), step)
                        .close();
                }

                if *part == FernPart::Stem {
                    let stemlet_width = Vec2::new(0.0015, 0.0);
                    builder
                        .begin(stemlet_width)
                        .line_to(step + stemlet_width)
                        .line_to(step - stemlet_width)
                        .line_to(-stemlet_width)
                        .close();
                }

                builder.translate(step);
                // p += step;
            }
            builder.pop();
        }

        let leaflets = settings.leaflets1;
        let mut px = 0.1;
        for i in 0..leaflets {
            let prog = i as f32 / leaflets as f32;
            let l0 = settings.l0;
            let leaflet_len = 1.0 - prog.powf(settings.leafshape_exp);
            let dir = ((i % 2) * 2) as f32 - 1.0;
            leaflet(
                Vec2::new(px, dir * (stem_w * (1.0 - prog) + stem_w2 * prog)),
                settings.leaflets2,
                leaflet_len,
                settings.curvature * (1.0 - 0.5 * prog),
                l0,
                dir,
                builder,
                settings,
                &part,
            );
            px += l0 * leaflet_len * settings.leaflet_spacing * 0.5;
        }

        //builder.quadratic_bezier_to(point(0.0, 1.0), point(1.0, 1.0));
        //builder.quadratic_bezier_to(point(1.0, 0.0), point(0.0, 0.0));
    });
    let mut fern = fill.build(false);
    fern.translate(0.5, 0.0, 0.0)
        .scale(settings.width as f32, settings.height as f32 / 2.0, 1.0);
    return fern;
}

#[no_mangle]
pub fn render_texture(
    width: u32,
    height: u32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    colors: [Color; 3],
    layer: u8,
    device: Res<bevy::render::renderer::RenderDevice>,
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

    return img;
}
