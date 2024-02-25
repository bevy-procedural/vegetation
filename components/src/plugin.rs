use bevy::{pbr::ExtendedMaterial, prelude::*, render::view::NoFrustumCulling};
use super::{FernMaterial, FernSettings};
use render_to_texture::{RenderToTexturePlugin, RenderToTextureTasks};

use crate::{make_fern_material, setup::make_fern_mesh};

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, make_fern_material);
        app.add_plugins(RenderToTexturePlugin)
            .add_systems(Startup, create_tasks)
            .add_systems(Update, (wait_for_texture, listen_for_changes));
    }
}

pub fn listen_for_changes(
    query: Query<&FernSettings, Changed<FernSettings>>,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
) {
    for _ in query.iter() {
        render_to_texture_tasks.get_mut("fern").unwrap().rerender();
    }
}

pub fn create_tasks(
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    render_to_texture_tasks.add(
        "fern".to_string(),
        2048,
        512,
        true,
        &mut commands,
        &mut images,
        true,
    );
}

#[derive(Component)]
struct FernMacroMesh;

fn wait_for_texture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FernMaterial>>>,
    mut render_to_texture_tasks: ResMut<RenderToTextureTasks>,
    mut images: ResMut<Assets<Image>>,
    _query: Query<&FernSettings>,
    mut macro_fern_query: Query<(Entity, &FernMacroMesh)>,
) {
    if let Some(image) = render_to_texture_tasks.image("fern", false) {
        // TODO: don't recreate the mesh! Better just change the texture. But how?
        /* for settings in query.iter() {
            println!("Got the image");
            *images
                .get_mut(settings.target_image.clone().unwrap())
                .unwrap() = image.clone();

        }*/

        // remove old
        for (entity, _) in macro_fern_query.iter_mut() {
            commands.entity(entity).despawn();
        }

        let material = make_fern_material(Some(images.add(image)), None);
        let mesh = make_fern_mesh();
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(material);

        for i in 0..30 {
            let s = (i as f32 * 100.0).sin() + 2.0;

            commands.spawn((
                MaterialMeshBundle {
                    mesh: mesh_handle.clone(),
                    transform: Transform::from_xyz(
                        ((1012.0 * i as f32).sin() * 100000.0) % 10.0,
                        s / 2.0,
                        ((432.0 * i as f32).sin() * 100000.0) % 10.0,
                    )
                    .with_scale(Vec3::splat(s)),
                    material: material_handle.clone(),
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
                FernMacroMesh,
            ));
        }
    }
}
