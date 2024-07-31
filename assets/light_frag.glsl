#version 450

layout(location = 0) in vec2 frag_uv;
layout(location = 1) in vec3 world_position;

layout(location = 0) out vec4 out_color;

layout(set = 2, binding = 0) uniform Material {
    vec4 material_color;
};

layout(set = 2, binding = 1) uniform LightCount {
    uint light_count;
};

layout(set = 2, binding = 2) buffer Lights {
    vec3 lights[];
};

void main() {
    bool discard_fragment = false;

    for (uint i = 0u; i < light_count; i++) {
        vec2 light_pos = lights[i].xy;
        float light_radius = lights[i].z;
        float lr_squared = light_radius * light_radius;
        vec2 diff = world_position.xy - light_pos;
        float distancesquared = dot(diff, diff);
        if (distancesquared < lr_squared) {
            discard_fragment = true;
            break;
        }
    }

    if (discard_fragment) {
        discard;
    }

    out_color = material_color;
}
