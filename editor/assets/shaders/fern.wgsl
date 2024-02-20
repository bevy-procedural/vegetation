#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}
// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/config.wgsl"::COLOR_MULTIPLIER

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: CustomMaterial;
@group(1) @binding(1) var base_color_texture: texture_2d<f32>;
@group(1) @binding(2) var base_color_sampler: sampler;
//@group(1) @binding(3) var<uniform> camera: Camera;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_normal: vec3<f32>
};

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.color * COLOR_MULTIPLIER;

    // let object_color: vec4<f32> = COLOR_MULTIPLIER; // textureSample(base_color_texture, base_color_sampler, mesh.uv);
    // let light_color = vec3<f32>(1.0, 1.0, 1.0); // light.color 
    // let light_position = vec3<f32>(0.0, 3.0, 3.0); // light.position

    // let ambient_strength = 0.1;
    // let ambient_color = light_color * ambient_strength;

    // let light_dir = normalize(light_position - mesh.clip_position.xyz);
    // let view_dir = normalize(camera.view_pos.xyz - mesh.clip_position.xyz);
    // let half_dir = normalize(view_dir + light_dir);

    // let diffuse_strength = max(dot(mesh.world_normal, light_dir), 0.0);
    // let diffuse_color = light_color * diffuse_strength;

    // let specular_strength = pow(max(dot(mesh.world_normal, half_dir), 0.0), 32.0);
    // let specular_color = specular_strength * light_color;

    // let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

    // return vec4<f32>(result, object_color.a);

    //return material.color * COLOR_MULTIPLIER; // * textureSample(base_color_texture, base_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var bendStrength = -0.05;
    var vertices_per_leaf: u32 = u32(12);
    var w = 0.2;

    var fi = f32(vertex.vertex_index % vertices_per_leaf) - 1.0;
    if fi <= 0.0 {
        fi = 0.0;
    } else if fi >= f32(vertices_per_leaf) - 3.0 {
        fi = f32(vertices_per_leaf) - 3.0;
    }
    var leaf = floor(f32(vertex.vertex_index) / f32(vertices_per_leaf));
    var yaw = -0.94 * leaf;
    var pitch = -0.9 + leaf * 0.02;

    var out: VertexOutput;
    var pos = vertex.position;
    var dist = floor(fi / 2.0);
    pos.x = -(fi % 2.0 - 0.5) * (f32(vertices_per_leaf) / 2.0 - 2.0 - dist) * w;
    var bentPitch = pitch + dist * bendStrength;
    pos.z += sin(bentPitch) * dist;
    pos.y += cos(bentPitch) * dist;

    // rotate around the y axis
    var x = pos.x;
    var z = pos.z;
    pos.x = x * cos(yaw) - z * sin(yaw);
    pos.z = x * sin(yaw) + z * cos(yaw);


    pos *= 0.2;

    let model = get_model_matrix(vertex.instance_index);
    out.clip_position = mesh_position_local_to_clip(model, vec4<f32>(pos, 0.0));
    return out;
}
