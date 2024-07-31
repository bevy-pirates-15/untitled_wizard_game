#version 450

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_uv;

layout(location = 0) out vec2 frag_uv;
layout(location = 1) out vec3 world_position;

layout(set = 0, binding = 0) uniform CameraData {
    mat4 view_proj;
    mat4 model;
};

void main() {
    frag_uv = in_uv;
    world_position = (model * vec4(in_position, 1.0)).xyz;
    gl_Position = view_proj * vec4(in_position, 1.0);
}
