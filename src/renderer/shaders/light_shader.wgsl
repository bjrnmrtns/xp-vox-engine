// validate shader cargo run --features wgsl-in -- ../../xp-vox-engine/src/renderer/shaders/light_shader.wgsl
[[block]]
struct Globals {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> u_globals: Globals;

[[block]]
struct Instance {
    models: array<mat4x4<f32>>;
};

[[group(0), binding(1)]]
var<storage> models: [[access(read)]] Instance;

struct VertexOutput {
    [[builtin(position)]] proj_position: vec4<f32>;
    [[location(0)]] world_position: vec3<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(instance_index)]] instance_idx: u32, [[location(0)]] model_position: vec3<f32>,
           [[location(1)]] model_normal: vec3<f32>,
           [[location(2)]] color: vec3<f32>) -> VertexOutput {
    let view = u_globals.view;
    let proj = u_globals.proj;
    let model = models.models[instance_idx];
    var out: VertexOutput;
    out.world_position = (model * vec4<f32>(model_position, 1.0)).xyz;
    // TODO: doing inverse for every vertex is expensive, this can be done once per mesh on cpu
    // TODO: fix (no inverse in wgsl -> out.world_normal = mat3(transpose(inverse(models[gl_InstanceIndex]))) * in_model_normal;
    // TODO: transpose(mat3x3<f32>(model.x.xyz, model.y.xyz, model.z.xyz));
    out.world_normal = model_normal;
    out.color = color;
    out.proj_position = proj * view * model * vec4<f32>(model_position, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let normal = normalize(in.world_normal);
    let result = vec3<f32>(1.0, 1.0, 1.0);
    let gamma: f32 = 2.2;
    //return vec4<f32>(pow(result, vec3<f32>(1.0 / gamma)), 1.0);
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}