use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix4, Point2, Point3, Point4, Vector3, Vector4};
use kiss3d::resource::Material;
use kiss3d::resource::{Effect, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::scene::ObjectData;

// Material which uses a flat color everywhere. Used for the skybox.
pub struct FlatMaterial {
    effect: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
    //tex: ShaderUniform<i32>,
    transform: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    color: ShaderUniform<Point4<f32>>,
}

impl FlatMaterial {
    pub fn new() -> FlatMaterial {
        // load the effect
        let mut effect = Effect::new_from_str(FLAT_VERTEX_SRC, FLAT_FRAGMENT_SRC);

        effect.use_program();

        // get the variables locations
        FlatMaterial {
            pos: effect.get_attrib("position").unwrap(),
            tex_coord: effect.get_attrib("tex_coord").unwrap(),
            //tex: effect.get_uniform("tex").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            color: effect.get_uniform("color").unwrap(),
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

impl Material for FlatMaterial {
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
        let formated_transform = transform.to_homogeneous()
            * Matrix4::from_diagonal(&Vector4::new(scale.x, scale.y, scale.z, 1.0));

        self.transform.upload(&formated_transform);

        mesh.bind_coords(&mut self.pos);
        mesh.bind_uvs(&mut self.tex_coord);
        mesh.bind_faces();

        ctxt.active_texture(Context::TEXTURE0);
        ctxt.bind_texture(Context::TEXTURE_2D, Some(&*data.texture()));
        self.color.upload(&Point4::new(
            data.color().x,
            data.color().y,
            data.color().z,
            1.0,
        ));

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

        mesh.coords().write().unwrap().unbind();
        mesh.uvs().write().unwrap().unbind();
        mesh.faces().write().unwrap().unbind();
        self.deactivate();
    }
}

/// Vertex shader used by the material to display line.
const FLAT_VERTEX_SRC: &str = "#version 100
    attribute vec3 position;
    attribute vec2 tex_coord;
    uniform   mat4 proj, view, transform;
    varying   vec2 frag_tex_coord;

    void main() {
        gl_Position = proj * view * transform * vec4(position, 1.0);
        frag_tex_coord = tex_coord;
    }";

/// Fragment shader used by the material to display line.
const FLAT_FRAGMENT_SRC: &str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec2 frag_tex_coord;
    uniform vec4 color;
    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, frag_tex_coord) * color;
    }";
