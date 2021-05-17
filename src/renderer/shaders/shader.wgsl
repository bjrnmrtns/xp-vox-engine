// validate shader cargo run --features wgsl-in -- ../../xp-vox-engine/src/renderer/shaders/light_shader.wgsl

struct DirectionalLight
{
    direction: vec4<f32>;
    ambient: vec4<f32>;
    diffuse: vec4<f32>;
    specular: vec4<f32>;
};

struct SpotLight
{
    position: vec4<f32>;
    direction: vec4<f32>;
    ambient: vec4<f32>;
    diffuse: vec4<f32>;
    specular: vec4<f32>;
    cons: f32;
    linear: f32;
    quadratic: f32;
    cut_off_inner: f32;
    cut_off_outer: f32;
    p0: f32; p1: f32; p2: f32;
};

struct PointLight
{
    position: vec4<f32>;
    ambient: vec4<f32>;
    diffuse: vec4<f32>;
    specular: vec4<f32>;
    cons: f32;
    linear: f32;
    quadratic: f32;
    p0: f32;
};

[[block]]
struct Globals {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
    world_camera_position: vec4<f32>;
    material_specular: vec4<f32>;
    material_shininess: f32;
    nr_of_directional_lights: u32;
    nr_of_spot_lights: u32;
    nr_of_point_lights: u32;
};

[[group(0), binding(0)]]
var<uniform> u_globals: Globals;

[[block]]
struct DirectionalLights {
    directional_lights: array<DirectionalLight, 1>;
};

[[group(0), binding(1)]]
var<uniform> directional_lights: DirectionalLights;

[[block]]
struct SpotLights {
    lights: array<SpotLight, 10>;
};

[[group(0), binding(2)]]
var<uniform> spot_lights: SpotLights;

[[block]]
struct PointLights {
    lights: array<PointLight, 10>;
};

[[group(0), binding(3)]]
var<uniform> point_lights: PointLights;

struct Instance {
    model: mat4x4<f32>;
};

[[block]]
struct Instances {
    models: array<Instance>;
};

[[group(0), binding(4)]]
var<storage> models: [[access(read)]] Instances;

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
    let model = models.models[instance_idx].model;
    var out: VertexOutput;
    out.proj_position = proj * view * model * vec4<f32>(model_position, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}