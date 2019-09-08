#version 330 core

in vec2 pos;
in vec2 tex;

out vec2 v_tex;

void main() {
    gl_Position = vec4(pos / 4.0 + 0.75, 0.0, 1.0);
    v_tex = tex;
}