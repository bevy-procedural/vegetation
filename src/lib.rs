use bevy::{prelude::*, render::mesh::shape::Cube, transform};
use components::*;
use procedural_meshes::{fill::MyFill, mesh::MyMesh, *};

#[no_mangle]
pub fn update_vegetation(
    query: Query<(&Children, Entity, &FernSettings)>,
    //q_name: Query<&Name>,
    mut assets: ResMut<Assets<Mesh>>,
) {
    for (children, _, settings) in query.iter() {
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

        // get children of entity
        /*for child in children.iter() {
            let name = q_name.get(*child).unwrap().as_str();
        }*/
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
            builder.begin(point(0.0, stem_w));
            builder.line_to(point(1.0, stem_w2));
            builder.line_to(point(1.0, -stem_w2));
            builder.line_to(point(0.0, -stem_w));
            builder.end(true);
        }

        fn leaflet(
            start: Point,
            leaflets: u32,
            leaflet_len: f32,
            curve: f32,
            l0: f32,
            dir: f32,
            builder: &mut fill::Builder,
            settings: &FernSettings,
            part: &FernPart,
        ) {
            let c0 = curve;
            let a0 = leaflet_len / ((leaflets + 1) as f32 * 0.5);
            let mut p = start;
            for i in 0..(leaflets - 2) {
                let prog = 1.0 - i as f32 / leaflets as f32;
                let l = l0 * prog * leaflet_len;
                let a = dir * a0 * prog;
                let step = vector(c0, a);
                let slant = settings.slant;
                let thinning = settings.thinning;
                let stomp = settings.stomp;

                if *part == FernPart::LeafletTop {
                    builder.begin(p);
                    builder.quadratic_bezier_to(
                        p + vector(l * stomp, thinning * a * (-0.5 + slant)),
                        p + vector(l, thinning * a * (0.5 + slant)),
                    );
                    builder
                        .quadratic_bezier_to(p + vector(l, thinning * a * (1.0 + slant)), p + step);
                    builder.end(true);
                }

                if *part == FernPart::LeafletBottom {
                    let l2 = -(l - 2.0 * c0);
                    builder.begin(p);
                    builder.quadratic_bezier_to(
                        p + vector(l2 * stomp, thinning * a * (-0.5 + slant)),
                        p + vector(l2, thinning * a * (0.5 + slant)),
                    );
                    builder.quadratic_bezier_to(
                        p + vector(l2, thinning * a * (1.0 + slant)),
                        p + step,
                    );
                    builder.end(true);
                }

                /*if *part == FernPart::Stem {
                    let stemlet_width = vector(0.0015, 0.0);
                    builder.begin(p + stemlet_width);
                    builder.line_to(p + step + stemlet_width);
                    builder.line_to(p + step - stemlet_width);
                    builder.line_to(p - stemlet_width);
                    builder.end(true);
                }*/

                p += step;
            }
        }

        let leaflets = settings.leaflets1;
        let mut px = 0.1;
        for i in 0..leaflets {
            let prog = i as f32 / leaflets as f32;
            let l0 = settings.l0;
            let leaflet_len = 1.0 - prog.powf(settings.leafshape_exp);
            let dir = ((i % 2) * 2) as f32 - 1.0;
            leaflet(
                point(px, dir * (stem_w * (1.0 - prog) + stem_w2 * prog)),
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
) -> Handle<Image> {
    let mut settings = FernSettings {
        width,
        height,
        ..default()
    };
    //let fern = fern_mesh(&settings);
    //let fern_mesh = fern.to_bevy();

    let (img, layer) = render_to_texture(width, height, commands, images, layer);
    let mesh = meshes.add(Mesh::from(Cube { size: 1.0 }));
    let mesh2 = meshes.add(Mesh::from(Cube { size: 1.0 }));
    let mesh3 = meshes.add(Mesh::from(Cube { size: 1.0 }));
    settings.meshes = vec![mesh.id(), mesh2.id(), mesh3.id()];

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
                    transform: Transform::from_xyz(0.0, -0.01, 0.0),
                    ..default()
                },
                layer,
            ));
            parent.spawn((
                ColorMesh2dBundle {
                    mesh: mesh3.into(),
                    material: materials.add(ColorMaterial::from(colors[2])),
                    transform: Transform::from_xyz(0.0, -0.01, 0.0),
                    ..default()
                },
                layer,
            ));
        });
    return img;
}
