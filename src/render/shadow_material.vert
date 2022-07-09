#version 100
attribute vec3 position;
attribute vec2 tex_coord;
attribute vec3 normal;

uniform mat3 ntransform, scale;
uniform mat4 proj, view, transform;

// frag_pos is set to the position of the vertex, in object space.
varying vec2 frag_tex_coord;
varying vec3 frag_pos;
varying vec3 frag_normal;

void main() {
    vec4 vert_pos4 = transform * vec4(scale * position, 1.0);
    frag_pos = vec3(vert_pos4) / vert_pos4.w;
    frag_normal = ntransform * normal;
    gl_Position = proj * view * vert_pos4;
    frag_tex_coord = tex_coord;
}