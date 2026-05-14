#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec4 color;
varying lowp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    // Macroquad sends color as bytes (0..255). 
    // We MUST divide by 255.0 to get 0.0..1.0 range.
    color = color0 / 255.0;
    uv = texcoord;
}
