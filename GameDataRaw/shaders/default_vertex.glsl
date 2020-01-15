#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec3 normal;
layout (location = 3) in vec3 tangent;
layout (location = 4) in vec3 bitangent;
layout (location = 5) in ivec4 b_index;
layout (location = 6) in vec4 b_weight;

out vec3 v_normal;
out vec2 v_tex_coords;
out vec3 v_position;
out vec4 v_shadow_coords;
out mat3 tbn;
out vec4 v_bones;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 matrix;
uniform mat4 depth_bias_mvp;
uniform mat4 bones[10];

void main() {
    vec4 b_position = vec4(0.0);
    mat4 modelview = view * matrix;
    vec3 t = normalize(tangent - normal * dot(normal, tangent));
    tbn = transpose(mat3(
        normalize(mat3(modelview) * t),
        normalize(mat3(modelview) * bitangent),
        normalize(mat3(modelview) * normal)
    ));
    mat4 bone;
    float weight;
    for (int i=0; i<4; i++) {
        if (b_index[i] != 255) {
            bone = bones[b_index[i]];
            weight = b_weight[i];
            b_position += bone * vec4(position, 1.0) * weight;
        }
    }
    v_bones = vec4(b_index);
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    v_shadow_coords = depth_bias_mvp * vec4(b_position.xyz + v_normal * 0.01, 1.0);
    gl_Position = perspective * modelview * vec4(b_position.xyz, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}