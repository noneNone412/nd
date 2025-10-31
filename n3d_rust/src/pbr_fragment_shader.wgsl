struct FSInput {
    @location(0) frag_pos  : vec3<f32>,   // world position
    @location(1) frag_uv   : vec2<f32>,   // texture coordinates
    @location(2) frag_norm : vec3<f32>,   // normal in world space
    @location(3) frag_col  : vec4<f32>,   // vertex color
};

// === Camera + model uniform (from vertex shader UBO) ===
struct Uniforms {
    viewProj   : mat4x4<f32>,
    model      : mat4x4<f32>,
    cameraPos  : vec3<f32>,
    _pad       : f32, // pad to 16 bytes
};
@group(0) @binding(0)
var<uniform> uniforms : Uniforms;

// === Material uniforms (factors from glTF) ===
struct Material {
    baseColorFactor : vec4<f32>,
    metallicFactor  : f32,
    roughnessFactor : f32,
    emissiveFactor  : vec3<f32>,
};
@group(1) @binding(0)
var<uniform> material : Material;

// === Textures + samplers ===
@group(1) @binding(1) var baseColorTex : texture_2d<f32>;
@group(1) @binding(2) var baseColorSampler : sampler;
@group(1) @binding(3) var metallicRoughnessTex : texture_2d<f32>;
@group(1) @binding(4) var metallicRoughnessSampler : sampler;
@group(1) @binding(5) var normalTex : texture_2d<f32>;
@group(1) @binding(6) var normalSampler : sampler;
@group(1) @binding(7) var occlusionTex : texture_2d<f32>;
@group(1) @binding(8) var occlusionSampler : sampler;
@group(1) @binding(9) var emissiveTex : texture_2d<f32>;
@group(1) @binding(10) var emissiveSampler : sampler;

// === Directional light ===
struct Light {
    direction : vec3<f32>,
    color     : vec3<f32>,
};
@group(2) @binding(0)
var<uniform> light : Light;

@fragment
fn fs_main(input : FSInput) -> @location(0) vec4<f32> {
    // --- 1. Base color ---
    var baseColor = material.baseColorFactor;
    baseColor = textureSample(baseColorTex, baseColorSampler, input.frag_uv);
    if (any(input.frag_col.rgb != vec3<f32>(0.0))) {
        baseColor *= input.frag_col;
    }

    // --- 2. Metallic + Roughness ---
    let mrSample = textureSample(metallicRoughnessTex, metallicRoughnessSampler, input.frag_uv);
    let metallic  = material.metallicFactor * mrSample.b;
    let roughness = material.roughnessFactor * mrSample.g;

    // --- 3. Normal mapping ---
    var N = normalize(input.frag_norm);
    let normalSample = textureSample(normalTex, normalSampler, input.frag_uv).rgb * 2.0 - 1.0;
    if (length(normalSample) > 0.01) {
        N = normalize(normalSample);
    }

    // --- 4. View and light vectors (camera-aware) ---
    let V = normalize(uniforms.cameraPos - input.frag_pos); // correct view vector
    let L = normalize(-light.direction);                   // directional light
    let H = normalize(V + L);

    let NdotL = max(dot(N, L), 0.0);
    let NdotV = max(dot(N, V), 0.0);
    let NdotH = max(dot(N, H), 0.0);
    let VdotH = max(dot(V, H), 0.0);

    // --- 5. Fresnel-Schlick approximation ---
    let F0 = mix(vec3<f32>(0.04, 0.04, 0.04), baseColor.rgb, metallic);
    let F = F0 + (1.0 - F0) * pow(1.0 - VdotH, 5.0);

    // --- 6. Normal Distribution (GGX simplified) ---
    let alpha = roughness * roughness;
    let denom = (NdotH * NdotH) * (alpha * alpha - 1.0) + 1.0;
    let D = alpha * alpha / (3.14159 * denom * denom);

    // --- 7. Geometry term (Schlick-GGX) ---
    let k = (alpha + 1.0) * (alpha + 1.0) / 8.0;
    let Gv = NdotV / (NdotV * (1.0 - k) + k);
    let Gl = NdotL / (NdotL * (1.0 - k) + k);
    let G = Gv * Gl;

    // --- 8. Cook-Torrance BRDF ---
    let numerator = D * G * F;
    let specular = numerator / max(4.0 * NdotV * NdotL, 0.001);

    let kS = F;                       // specular reflection
    var kD = vec3<f32>(1.0) - kS;     // diffuse reflection
    kD *= 1.0 - metallic;             // metals have less diffuse
    let diffuse = kD * baseColor.rgb / 3.14159;

    var finalColor = (diffuse + specular) * light.color * NdotL;
    

    // --- 9. Occlusion ---
    let ao = textureSample(occlusionTex, occlusionSampler, input.frag_uv).r;
    finalColor *= ao;

    // --- 10. Emissive ---
    let emissive = material.emissiveFactor * textureSample(emissiveTex, emissiveSampler, input.frag_uv).rgb;
    finalColor += emissive;

    return vec4<f32>(finalColor, baseColor.a);
}
