#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

varying vec3 frag_pos;
varying vec3 frag_normal;
varying vec2 frag_tex_coord;

uniform vec3 color;
uniform sampler2D tex;

uniform vec3 light_pos;
//uniform float light_radius;
uniform vec3 occluder_pos;
uniform float occluder_radius;

float segment_intersects_sphere(vec3 start, vec3 end, vec3 center, float radius) {
    vec3 seg = end - start;
    float t = dot(center - start, seg) / dot(seg, seg);
    t = clamp(t, 0.0, 1.0);
    vec3 closest_point = start + seg * t;
    vec3 to_center = center - closest_point;
    
    float sqdist = dot(to_center, to_center);
    float sqradius = radius * radius;
    return clamp((sqdist/sqradius - 1.0) / (1.05*1.05), 0.0, 1.0);
}

void main() {
  vec3 normal = normalize(frag_normal);
  vec3 light_dir = normalize(light_pos - frag_pos);

  float lambertian = max(dot(light_dir, normal), 0.0);
  lambertian *= segment_intersects_sphere(frag_pos, light_pos, occluder_pos, occluder_radius);

  vec4 tex_color = texture2D(tex, frag_tex_coord);
  gl_FragColor = tex_color * vec4(color * (0.2 + lambertian), 1.0);
}