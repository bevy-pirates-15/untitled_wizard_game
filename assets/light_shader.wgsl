#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> light_count: u32;
@group(2) @binding(2) var<uniform> lights: array<vec3<f32>, 64>;


@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    var world_pos = in.world_position;
    var discard_fragment = false;

    for (var i = 0u; i < light_count; i = i + 1u) {
        let light_pos = lights[i].xy;
        let light_radius = lights[i].z;
        let lr_squared = light_radius * light_radius;
        let diff = world_pos.xy - light_pos;
        let distancesquared = dot(diff, diff);
        if (distancesquared < lr_squared) {
            discard_fragment = true;
            break;
        }
    }

    if (discard_fragment) {
        discard;
    }

    return material_color;
}




