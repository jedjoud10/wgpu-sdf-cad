#version 460 core

layout(location = 0) out vec4 fragColor;

void main() {
    vec2 pos = gl_Position.xy;
    fragColor = vec4(pos, 0.0, 1.0);
}