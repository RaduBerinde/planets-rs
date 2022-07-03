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
uniform float light_radius;
uniform vec3 occluder_pos;
uniform float occluder_radius;

// To create softer shadows, full shadow is only when a ray intersects an occluder that this much smaller.
const float full_shadow_radius_fraction = 0.90;
const float full_shadow_radius_fraction_sq = full_shadow_radius_fraction * full_shadow_radius_fraction;

float point_source_shadow(vec3 light_vec, vec3 occluder_vec, float occluder_radius) {
    // We want to determine the point on the segment that is closest to the
    // sphere center. Project the center point onto the line and normalize
    // distance from start to [0, 1].
    float t = clamp(dot(occluder_vec, light_vec) / dot(light_vec, light_vec), 0.0, 1.0);

    vec3 closest_point = light_vec * t;
    vec3 to_center = occluder_vec - closest_point;
    
    float sqdist = dot(to_center, to_center);
    float sqradius = occluder_radius * occluder_radius;
    
    // sqdist <= sqradius * sqfraction     => 0.0
    // sqdist >= sqradis                   => 1.0
    // 
    // blended = (sqdist - sqradius*sqfraction) / (sqradius - sqradius * sqfraction)
    //         = (sqdist - sqradius*sqfraction) / (sqradius * (1 - sqfraction))
    //         = (sqdist/sradius - sqfraction) / (1 - sqfraction)
    
    return clamp((sqdist/sqradius - full_shadow_radius_fraction_sq) / (1.0 - full_shadow_radius_fraction_sq), 0.0, 1.0);
}

float spherical_source_shadow(vec3 light_vec, float light_vec_len, float light_radius, vec3 occluder_vec, float occluder_radius) {
   float occluder_vec_len = length(occluder_vec);
   // If occluder is farther than the light source,there is no shadow.
   // Note: we assume the objects in question don't intersect.
   if (occluder_vec_len > light_vec_len) {
      return 1.0;
   }
   // If the occluder is behind (in the direction of the light source), there is no shadow.
   // Note: we should really check if it's behind by more than the occluder radius,
   // but this only makes a difference with objects that are very close to each other.
   if (dot(occluder_vec, light_vec) <= 0.0) {
      return 1.0;
   }
   
   // Consider the cone with apex at origin and height light_vec. Consider the
   // projection of the cone in the plane defined by the three points (origin,
   // light, occluder).
   // The height of the cone is light_vec_len and the base radius is light_radius.

   // We first want to check if there is any potential intersection between
   // this cone and the sphere. To calculate this, we move the cone back (so
   // that the new cone surface is occluder_radius away from the old surface)
   // and check if that extended cone contains the occluder center. We have to
   // move the cone back by occluder_radius * light_vec_len / light_radius.
   vec3 extended_cone_apex = - light_vec * (occluder_radius / light_radius);
   float cos_phi = light_vec_len / sqrt(light_vec_len*light_vec_len + light_radius*light_radius);
   if (dot(normalize(light_vec - extended_cone_apex), normalize(occluder_vec - extended_cone_apex)) < cos_phi) {
      return 1.0;
   }
   
   vec3 axis1 = normalize(cross(light_vec, vec3(0.0, 0.0, 1.0)));
   vec3 axis2 = normalize(cross(light_vec, axis1));
   
   const int num_rings = 32;
   const int samples_per_ring = 32;
   float u_step = 1.0 / float(num_rings-1);
   float v_step = 2.0 * 3.141592 / float(samples_per_ring);
   
   float result = 0.0;
   float u = 0.0;
   for (int i = 0; i < num_rings; i++) {
      float v = 0.0;
      for (int j = 0; j < samples_per_ring; j++) {
         float r = light_radius * sqrt(u);
         vec3 light_sample_pos = light_vec + r * (axis1 * sin(v) + axis2 * cos(v));
         result += point_source_shadow(light_sample_pos, occluder_vec, occluder_radius);
         v += v_step;
      }
      u += u_step;
   }
   
   return result / float(num_rings * samples_per_ring);
}

void main() {
  vec3 normal = normalize(frag_normal);
  vec3 light_vec = light_pos - frag_pos;
  float light_vec_len = length(light_vec);
  //vec3 light_dir = normalize(light_pos - frag_pos);

  float lambertian = max(dot(light_vec, normal) / light_vec_len, 0.0);
  if (lambertian > 0.0) {
     //lambertian *= point_source_shadow(light_vec, occluder_pos - frag_pos, occluder_radius);
     lambertian *= spherical_source_shadow(light_vec, light_vec_len, light_radius, occluder_pos - frag_pos, occluder_radius);
  }

  vec4 tex_color = texture2D(tex, frag_tex_coord);
  gl_FragColor = tex_color * vec4(color * (0.2 + lambertian), 1.0);
}