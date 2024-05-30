#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var tilemap_texture: texture_2d<f32>;
@group(2) @binding(2) var tilemap_texture_sampler: sampler;
@group(2) @binding(3) var<uniform> tile_data: array<vec2>;
@group(2) @binding(4) var<uniform> map_width: u32;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return textureSample(tilemap_texture, tilemap_texture_sampler, mesh.uv);
}