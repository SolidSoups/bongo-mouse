struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}


@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var<uniform> sprite_rect: vec4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let window_pos = sprite_rect.xy + in.position * sprite_rect.zw;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(window_pos.x * 2.0 - 1.0, 1.0 - window_pos.y * 2.0, 0.0, 1.0);
    out.tex_coords = in.tex_coords;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return vec4<f32>(color.rgb * color.a, color.a);
}
