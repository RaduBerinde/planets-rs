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

//void main() {
//  gl_FragColor = vec4(color * frag_tex_coord.x, 1.0);
//}
void main() {
  vec3 normal = normalize(frag_normal);
  vec3 light_position;
  vec3 light_dir = normalize(light_position - frag_pos);

  float lambertian = max(dot(light_dir, normal), 0.0);

  vec4 tex_color = texture2D(tex, frag_tex_coord);
  gl_FragColor = tex_color * vec4(color * (0.2 + lambertian), 1.0);
}
