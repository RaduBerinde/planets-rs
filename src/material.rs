use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix3, Matrix4, Point2, Point3, Vector3};
use kiss3d::resource::Material;
use kiss3d::resource::{Effect, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::scene::ObjectData;

/// The default material used to draw objects.
pub struct MyMaterial {
    effect: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
    light: ShaderUniform<Point3<f32>>,
    color: ShaderUniform<Point3<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
    ntransform: ShaderUniform<Matrix3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
}

impl MyMaterial {
    /// Creates a new `MyMaterial`.
    pub fn new() -> MyMaterial {
        // load the effect
        let mut effect = Effect::new_from_str(OBJECT_VERTEX_SRC, OBJECT_FRAGMENT_SRC);

        effect.use_program();

        // get the variables locations
        MyMaterial {
            pos: effect.get_attrib("position").unwrap(),
            normal: effect.get_attrib("normal").unwrap(),
            tex_coord: effect.get_attrib("tex_coord").unwrap(),
            light: effect.get_uniform("light_position").unwrap(),
            color: effect.get_uniform("color").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            scale: effect.get_uniform("scale").unwrap(),
            ntransform: effect.get_uniform("ntransform").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            effect,
        }
    }

    fn activate(&mut self) {
        self.effect.use_program();
        self.pos.enable();
        self.normal.enable();
        self.tex_coord.enable();
    }

    fn deactivate(&mut self) {
        self.pos.disable();
        self.normal.disable();
        self.tex_coord.disable();
    }
}

impl Material for MyMaterial {
    fn render(
        &mut self,
        pass: usize,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        camera: &mut dyn Camera,
        light: &Light,
        data: &ObjectData,
        mesh: &mut Mesh,
    ) {
        let ctxt = Context::get();
        self.activate();

        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        let pos = match *light {
            Light::Absolute(ref p) => *p,
            Light::StickToCamera => camera.eye(),
        };

        self.light.upload(&pos);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_ntransform = transform.rotation.to_rotation_matrix().into_inner();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        self.transform.upload(&formated_transform);
        self.ntransform.upload(&formated_ntransform);
        self.scale.upload(&formated_scale);

        mesh.bind(&mut self.pos, &mut self.normal, &mut self.tex_coord);

        ctxt.active_texture(Context::TEXTURE0);
        ctxt.bind_texture(Context::TEXTURE_2D, Some(&*data.texture()));

        if data.surface_rendering_active() {
            self.color.upload(data.color());

            if data.backface_culling_enabled() {
                ctxt.enable(Context::CULL_FACE);
            } else {
                ctxt.disable(Context::CULL_FACE);
            }

            let _ = ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
            ctxt.draw_elements(
                Context::TRIANGLES,
                mesh.num_pts() as i32,
                Context::UNSIGNED_SHORT,
                0,
            );
        }

        if data.lines_width() != 0.0 {
            self.color
                .upload(data.lines_color().unwrap_or(data.color()));

            ctxt.disable(Context::CULL_FACE);
            ctxt.line_width(data.lines_width());

            if ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::LINE) {
                ctxt.draw_elements(
                    Context::TRIANGLES,
                    mesh.num_pts() as i32,
                    Context::UNSIGNED_SHORT,
                    0,
                );
            } else {
                mesh.bind_edges();
                ctxt.draw_elements(
                    Context::LINES,
                    mesh.num_pts() as i32 * 2,
                    Context::UNSIGNED_SHORT,
                    0,
                );
            }
            ctxt.line_width(1.0);
        }

        if data.points_size() != 0.0 {
            self.color.upload(data.color());

            ctxt.disable(Context::CULL_FACE);
            ctxt.point_size(data.points_size());
            if ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::POINT) {
                ctxt.draw_elements(
                    Context::TRIANGLES,
                    mesh.num_pts() as i32,
                    Context::UNSIGNED_SHORT,
                    0,
                );
            } else {
                ctxt.draw_elements(
                    Context::POINTS,
                    mesh.num_pts() as i32,
                    Context::UNSIGNED_SHORT,
                    0,
                );
            }
            ctxt.point_size(1.0);
        }

        mesh.unbind();
        self.deactivate();
    }
}

const OBJECT_VERTEX_SRC: &str = "
#version 100
attribute vec3 position;
attribute vec2 tex_coord;
attribute vec3 normal;

uniform mat3 ntransform, scale;
uniform mat4 proj, view, transform;
uniform vec3 light_position;

varying vec3 local_light_position;
varying vec2 tex_coord_v;
varying vec3 normalInterp;
varying vec3 vertPos;
varying vec3 uv_as_a_color;

void main(){
    gl_Position = proj * view * transform * vec4(scale * position, 1.0);
    vec4 vertPos4 = view * transform * vec4(scale * position, 1.0);
    vertPos = vec3(vertPos4) / vertPos4.w;
    normalInterp = mat3(view) * ntransform * normal;
    tex_coord_v = tex_coord;
    local_light_position = (view * vec4(light_position, 1.0)).xyz;
}
";

const OBJECT_FRAGMENT_SRC: &str = "
#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

varying vec3 local_light_position;
varying vec2 tex_coord_v;
varying vec3 normalInterp;
varying vec3 vertPos;

uniform vec3 color;
uniform sampler2D tex;
const vec3 specColor = vec3(0.4, 0.4, 0.4);

void main() {
  vec3 normal = normalize(normalInterp);
  vec3 lightDir = normalize(local_light_position - vertPos);

  float lambertian = max(dot(lightDir, normal), 0.0);
  float specular = 0.0;

  if(lambertian > 0.0) {
    vec3 viewDir = normalize(-vertPos);
    vec3 halfDir = normalize(lightDir + viewDir);
    float specAngle = max(dot(halfDir, normal), 0.0);
    specular = pow(specAngle, 30.0);
  }

  vec4 tex_color = texture2D(tex, tex_coord_v);
  gl_FragColor = vec4(color * (0.5 + tex_coord_v.x), 1.0) + /*vec4((normal + 1.0) / 2.0, 1.0) + */0.001 * tex_color * vec4(color / 3.0 +
                                  lambertian * color / 3.0 +
                                  specular * specColor / 3.0, 0.0);
}
";
