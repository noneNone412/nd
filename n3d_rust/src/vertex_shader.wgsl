struct Uniforms {
    viewProj : mat4x4<f32>,
    model    : mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms : Uniforms;

struct VSInput {
    @location(0) position : vec3<f32>,
    @location(1) normal   : vec3<f32>,
    @location(2) tangent  : vec4<f32>,
    @location(3) uv0      : vec2<f32>,
    @location(4) color    : vec4<f32>,
    @location(5) joints   : vec4<u32>,
    @location(6) weights  : vec4<f32>,
};

struct VSOutput {
    @builtin(position) clip_pos : vec4<f32>,
    @location(0) frag_pos  : vec3<f32>,
    @location(1) frag_uv   : vec2<f32>,
    @location(2) frag_norm : vec3<f32>,
    @location(3) frag_col  : vec4<f32>,
};

@vertex
fn vs_main(input : VSInput) -> VSOutput {
    var output : VSOutput;

    // Transform vertex position
    let world_pos = uniforms.model * vec4<f32>(input.position, 1.0);
    output.clip_pos = uniforms.viewProj * world_pos;
    output.frag_pos = world_pos.xyz;

    // Transform normal (ignore inverse transpose)
    output.frag_norm = normalize((uniforms.model * vec4<f32>(input.normal, 0.0)).xyz);

    // Pass UVs and color
    output.frag_uv = input.uv0;
    output.frag_col = input.color;

    return output;
}
