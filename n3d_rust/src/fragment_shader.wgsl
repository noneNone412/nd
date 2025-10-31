// Input from the vertex shader
struct FSInput {
    @location(0) frag_uv : vec2<f32>,
};

// Output a single color
@fragment
fn fs_main(input: FSInput) -> @location(0) vec4<f32> {
    // Simple gradient based on UV coordinates
    let color = vec4<f32>(input.frag_uv.x, input.frag_uv.y, 0.5, 1.0);
    return color;
}
