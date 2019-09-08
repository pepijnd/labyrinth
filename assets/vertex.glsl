#version 330 core

in vec3 position;
in vec2 tex_coords;
in vec3 normal;

out vec3 v_normal;
out vec2 v_tex_coords;
out vec3 v_position;
out vec4 v_shadow_coords;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 matrix;
uniform mat4 depth_bias_mvp;

void main() {
    mat4 modelview = view * matrix;
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    v_shadow_coords = depth_bias_mvp * vec4(position, 1.0);
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}