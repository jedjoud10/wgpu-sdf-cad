#version 460 core

const vec2 VERTICES[6] = {
    vec2(-1.0, -1.0),
    vec2(1.0, -1.0),
    vec2(-1.0, 1.0),
    vec2(-1.0, 1.0),
    vec2(1.0, -1.0),
    vec2(1.0, 1.0),
};

void main() {
    gl_Position = vec4(VERTICES[gl_VertexIndex], 0.0, 1.0);
}