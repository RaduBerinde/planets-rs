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
    tex_coord: ShaderAttribute<Point2<f32>>,
    color: ShaderUniform<Point3<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
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
            tex_coord: effect.get_attrib("tex_coord").unwrap(),
            color: effect.get_uniform("color").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            scale: effect.get_uniform("scale").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            effect,
        }
    }

    fn activate(&mut self) {
        self.effect.use_program();
        self.pos.enable();
        self.tex_coord.enable();
    }

    fn deactivate(&mut self) {
        self.pos.disable();
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
        _: &Light,
        data: &ObjectData,
        mesh: &mut Mesh,
    ) {
        let ctxt = Context::get();
        self.activate();

        /*
         *
         * Setup camera.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        self.transform.upload(&formated_transform);
        self.scale.upload(&formated_scale);

        //mesh.bind(&mut self.pos, &mut self.normal, &mut self.tex_coord);
        mesh.bind_coords(&mut self.pos);
        mesh.bind_uvs(&mut self.tex_coord);
        mesh.bind_faces();

        //ctxt.active_texture(Context::TEXTURE0);
        //ctxt.bind_texture(Context::TEXTURE_2D, Some(&*data.texture()));

        self.color.upload(data.color());

        ctxt.enable(Context::CULL_FACE);
        let _ = ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL);
        ctxt.draw_elements(
            Context::TRIANGLES,
            mesh.num_pts() as i32,
            Context::UNSIGNED_SHORT,
            0,
        );

        mesh.unbind();
        self.deactivate();
    }
}

const OBJECT_VERTEX_SRC: &str = "
#version 100
attribute vec3 position;
attribute vec2 tex_coord;

uniform mat3 scale;
uniform mat4 proj, view, transform;

varying vec2 tex_coord_v;

void main(){
    gl_Position = proj * view * transform * vec4(scale * position, 1.0);
    tex_coord_v = tex_coord;
}
";

const OBJECT_FRAGMENT_SRC: &str = "
#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

varying vec2 tex_coord_v;

uniform vec3 color;

void main() {
  gl_FragColor = vec4(color * tex_coord_v.x, 1.0);
}
";
