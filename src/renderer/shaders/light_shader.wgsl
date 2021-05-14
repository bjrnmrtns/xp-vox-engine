[[block]]
struct Globals {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> u_globals: Globals;

[[block]]
struct InstancesBuffer {
    model: array<mat4x4<f32>>;
}

[[group(0), binding(1)]]
var<storage> models: [[access(read)]] InstancesBuffer;

struct VertexOutput {
    [[builtin(position)]] proj_position: vec4<f32>;
    [[location(0)]] world_position: vec3<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] color: vec3<f32>;
};

[[stage(vertex]]
fn vs_main([[builtin(instance_index)]] instance_idx: u32, [[location(0)]] model_position: vec3<f32>,
           [[location(1)]] model_normal: vec3<f32>,
           [[location(2)]] color: vec3<f32>) -> VertexOutput {
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {

}