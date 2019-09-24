#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec3 normal;
layout (location = 3) in vec3 tangent;
layout (location = 4) in vec3 bitangent;

out vec3 v_normal;
out vec2 v_tex_coords;
out vec3 v_position;
out vec4 v_shadow_coords;
out mat3 tbn;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 matrix;
uniform mat4 depth_bias_mvp;

void main() {
    mat4 modelview = view * matrix;
    vec3 t = normalize(tangent - normal * dot(normal, tangent));
    tbn = transpose(mat3(
        normalize(mat3(modelview) * t),
        normalize(mat3(modelview) * bitangent),
        normalize(mat3(modelview) * normal)
    ));
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    v_shadow_coords = depth_bias_mvp * vec4(position + v_normal * 0.01, 1.0);
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}