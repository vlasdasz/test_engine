
@vertex
fn v_main(
    @location(0) position: vec2<f32>,
) -> @builtin(position) vec4<f32>  {
    return vec4<f32>(position, 1.0, 1.0);
}

@group(0) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return color;
}