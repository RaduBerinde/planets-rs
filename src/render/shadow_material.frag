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

const float pi = 3.141592653589793238;

//// To create softer shadows, full shadow is only when a ray intersects an occluder that this much smaller.
//const float full_shadow_radius_fraction = 0.95;
//const float full_shadow_radius_fraction_sq = full_shadow_radius_fraction * full_shadow_radius_fraction;
//
//float point_source_shadow(vec3 light_vec, vec3 occluder_vec, float occluder_radius) {
//    // We want to determine the point on the segment that is closest to the
//    // sphere center. Project the center point onto the line and normalize
//    // distance from start to [0, 1].
//    float t = clamp(dot(occluder_vec, light_vec) / dot(light_vec, light_vec), 0.0, 1.0);
//
//    vec3 closest_point = light_vec * t;
//    vec3 to_center = occluder_vec - closest_point;
//    
//    float sqdist = dot(to_center, to_center);
//    float sqradius = occluder_radius * occluder_radius;
//    
//    // sqdist <= sqradius * sqfraction     => 0.0
//    // sqdist >= sqradis                   => 1.0
//    // 
//    // blended = (sqdist - sqradius*sqfraction) / (sqradius - sqradius * sqfraction)
//    //         = (sqdist - sqradius*sqfraction) / (sqradius * (1 - sqfraction))
//    //         = (sqdist/sradius - sqfraction) / (1 - sqfraction)
//    
//    return clamp((sqdist/sqradius - full_shadow_radius_fraction_sq) / (1.0 - full_shadow_radius_fraction_sq), 0.0, 1.0);
//}

// circle_circle_intersection returns the area of the intersection
// between the unit circle and a circle of radius r with the center
// at distance d from the unit circle center.
float circle_circle_intersection(float r, float d) {
   const float eps = 0.001;
   if (d > r+1.0-eps) {
      return 0.0;
   }
   if (d < eps || d+r < 1.0+eps || d+1.0 < r+eps) {
      return pi * min(r, 1.0)*min(r, 1.0);
   }
   return r*r*acos((d*d+r*r-1.0) / (2.0*d*r)) + acos((d*d+1.0-r*r) / (2.0*d)) - 0.5*sqrt((-d+r+1.0)*(d+r-1.0)*(d-r+1.0)*(d+r+1.0));
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
   float light_edge_len = sqrt(light_vec_len*light_vec_len + light_radius*light_radius);
   float cos_phi = light_vec_len / light_edge_len;
   if (dot(normalize(light_vec - extended_cone_apex), normalize(occluder_vec - extended_cone_apex)) < cos_phi) {
      return 1.0;
   }

   // Consider the plane perpendicular to light_vec. We project the occluder
   // onto this plane, pretending that the projection is a circle. In general,
   // the projection on an arbitrary plane is not a circle but it is a good
   // approximation for our purpose (our light cone angles are small).
   
   vec3 occluder_vec_dir = occluder_vec / occluder_vec_len;
   vec3 light_vec_dir = light_vec / light_vec_len;

   float cos_theta = dot(occluder_vec_dir, light_vec_dir);
   float sin_theta = length(cross(occluder_vec_dir, light_vec_dir));
   float projected_distance = light_vec_len / cos_theta;
   float projected_distance_to_light = projected_distance * sin_theta;
   float projected_radius = occluder_radius / occluder_vec_len * projected_distance;
   
   //if (projected_distance_to_light >= light_radius + projected_radius) {
   //   return 1.0;
   //}

   //if (projected_distance_to_light + projected_radius <= light_radius) {
   //   // Projected circle is contained in the light circle; return fraction of
   //   // visible area.
   //   return 1.0 - projected_radius*projected_radius / (light_radius*light_radius);
   //}
   float area = circle_circle_intersection(projected_radius / light_radius, projected_distance_to_light / light_radius);
   return 1.0 - clamp(area/pi, 0.0, 1.0);

   //vec3 axis1 = normalize(cross(light_vec, vec3(0.0, 0.0, 1.0)));
   //vec3 axis2 = normalize(cross(light_vec, axis1));
   //
   //const int num_rings = 16;
   //const int samples_per_ring = 16;
   //float u_step = 1.0 / float(num_rings-1);
   //float v_step = 2.0 * pi / float(samples_per_ring);
   //
   //float result = 0.0;
   //float u = 0.0;
   //for (int i = 0; i < num_rings; i++) {
   //   float v = 0.0;
   //   for (int j = 0; j < samples_per_ring; j++) {
   //      float r = light_radius * sqrt(u);
   //      vec3 light_sample_pos = light_vec + r * (axis1 * sin(v) + axis2 * cos(v));
   //      result += point_source_shadow(light_sample_pos, occluder_vec, occluder_radius);
   //      v += v_step;
   //   }
   //   u += u_step;
   //}
   //
   //return result / float(num_rings * samples_per_ring);
}

void main() {
  vec3 normal = normalize(frag_normal);
  vec3 light_vec = light_pos - frag_pos;
  float light_vec_len = length(light_vec);

  float lambertian = max(dot(light_vec, normal) / light_vec_len, 0.0);
  if (lambertian > 0.0) {
     //lambertian *= point_source_shadow(light_vec, occluder_pos - frag_pos, occluder_radius);
     lambertian *= spherical_source_shadow(light_vec, light_vec_len, light_radius, occluder_pos - frag_pos, occluder_radius);
  }

  vec4 tex_color = texture2D(tex, frag_tex_coord);
  gl_FragColor = tex_color * vec4(color * (0.2 + lambertian), 1.0);
}