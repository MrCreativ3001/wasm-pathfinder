#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform vec2 u_tile_size;
uniform vec2 u_position;

in vec2 a_position;

void main() {
    vec2 grid_position = (u_position + a_position) * u_tile_size;

    vec2 zero_to_one = grid_position / u_resolution;
    // we have to invert the y coordinate here because WebGl by default renders from the bottom left corner
    vec2 zero_to_two = vec2(zero_to_one.x * 2.0, zero_to_one.y * -2.0);
    vec2 clip_space = vec2(zero_to_two.x - 1.0, zero_to_two.y + 1.0);

    gl_Position = vec4(clip_space, 0.0, 1.0);
}