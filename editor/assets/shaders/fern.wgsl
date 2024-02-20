#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_world,mesh_position_local_to_clip}
#import "shaders/config.wgsl"::{fern_vertices, Vertex}
#import bevy_pbr::{mesh_view_bindings::globals, forward_io::VertexOutput}


struct CustomMaterial {
    time: f32,
};

@group(1) @binding(100) var<uniform> material: CustomMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let res = fern_vertices(globals.time, vertex);
    let model = get_model_matrix(vertex.instance_index);
    var out = VertexOutput();
    out.position = mesh_position_local_to_clip(model, vec4<f32>(res.pos, 1.0));
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(res.pos, 1.0));
    out.world_position = (model * vec4<f32>(res.pos, 1.0));
    out.world_normal = (model * vec4<f32>(res.normal, 0.0)).xyz;
    out.instance_index = vertex.instance_index;
    return out;
}
