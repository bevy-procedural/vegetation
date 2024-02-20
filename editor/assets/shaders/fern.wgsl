#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}
#import "shaders/config.wgsl"::COLOR_MULTIPLIER
#import bevy_pbr::{mesh_view_bindings::globals, forward_io::VertexOutput}


struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(100) var<uniform> material: CustomMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
};

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.color * COLOR_MULTIPLIER;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var vertices_per_leaf: u32 = u32(12);
    var w = 2.0 / f32(vertices_per_leaf);
    var h = 12.0 / f32(vertices_per_leaf);
    var bendStrength = -0.5 / f32(vertices_per_leaf);

  
    let tooth = (f32(vertex.vertex_index) / 2.0) % 2.0;
    if tooth <= 0.1 || tooth >= 1.4 {
        //w = w * 0.1;
    }

    var fi = f32(vertex.vertex_index % vertices_per_leaf) - 1.0;
    if fi <= 0.0 {
        fi = 0.0;
    } else if fi >= f32(vertices_per_leaf) - 3.0 {
        fi = f32(vertices_per_leaf) - 3.0;
    }
    
    var out: VertexOutput;
    var pos = vertex.position;
    var dist = floor(fi / 2.0);
    var leaf = floor(f32(vertex.vertex_index) / f32(vertices_per_leaf));

    var yaw = -0.94 * leaf;

    var time = globals.time - (yaw % radians(360.0)) - dist * 0.3;
    var wind = sin(time) - sin(time / 2.0) + sin(time / 4.0) - sin(time / 8.0);

    var pitch = -0.9 + leaf * 0.02 + wind * 0.08;

    pos.x = -(fi % 2.0 - 0.5) * (f32(vertices_per_leaf) / 2.0 - 2.0 - dist) * w;
    var bentPitch = pitch + dist * bendStrength;
    pos.y += cos(bentPitch) * dist * h;
    pos.z += sin(bentPitch) * dist * h;

    // rotate around the y axis
    var yaw_rotation = mat2x2<f32>(cos(yaw), sin(yaw), -sin(yaw), cos(yaw));
    var r = pos.xz * yaw_rotation;
    pos.x = r.x;
    pos.z = r.y;

    var normal = vec4<f32>(0.0, cos(bentPitch + radians(90.0)), sin(bentPitch + radians(90.0)), 0.0);
    var rr = normal.xz * yaw_rotation;
    normal.x = rr.x;
    normal.z = rr.y;

    pos *= 0.2;

    let model = get_model_matrix(vertex.instance_index);
    out.position = mesh_position_local_to_clip(model, vec4<f32>(pos, 0.0));
    out.world_position = (model * vec4<f32>(pos, 1.0)); // TODO
    out.world_normal = (model * normal).xyz;
    out.instance_index = vertex.instance_index;
    return out;
}
