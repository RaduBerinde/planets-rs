use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix4, Point2, Point3, Point4, Vector3, Vector4};
use kiss3d::resource::{AllocationType, BufferType, Effect, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::resource::{GPUVec, Material};
use kiss3d::scene::ObjectData;

pub struct LinesMaterial {
    effect: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderAttribute<Point4<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    lines_width: f32,
}

// LinesData is used as for object's generic data (inside a Rc RefCell).
pub struct LinesData {
    coords: GPUVec<Point3<f32>>,
    colors: GPUVec<Point4<f32>>,
    edges: GPUVec<Point2<u16>>,
}

impl LinesData {
    pub fn new(
        coords: Vec<Point3<f32>>,
        colors: Vec<Point4<f32>>,
        edges: Vec<Point2<u16>>,
    ) -> Self {
        let location = AllocationType::StaticDraw;
        LinesData {
            coords: GPUVec::new(coords, BufferType::Array, location),
            colors: GPUVec::new(colors, BufferType::Array, location),
            edges: GPUVec::new(edges, BufferType::ElementArray, location),
        }
    }

    pub fn bind(
        &mut self,
        coords: &mut ShaderAttribute<Point3<f32>>,
        colors: &mut ShaderAttribute<Point4<f32>>,
    ) {
        coords.bind(&mut self.coords);
        colors.bind(&mut self.colors);
        self.edges.bind();
    }

    pub fn unbind(&mut self) {
        self.coords.unbind();
        self.colors.unbind();
        self.edges.unbind();
    }
}

impl LinesMaterial {
    pub fn new() -> LinesMaterial {
        // load the effect
        let mut effect = Effect::new_from_str(LINES_VERTEX_SRC, LINES_FRAGMENT_SRC);

        effect.use_program();

        LinesMaterial {
            pos: effect.get_attrib("position").unwrap(),
            color: effect.get_attrib("color").unwrap(),
            transform: effect.get_uniform("transform").unwrap(),
            view: effect.get_uniform("view").unwrap(),
            proj: effect.get_uniform("proj").unwrap(),
            effect,
            lines_width: 1.0,
        }
    }

    fn activate(&mut self) {
        self.effect.use_program();
        self.pos.enable();
        self.color.enable();
    }

    fn deactivate(&mut self) {
        self.pos.disable();
        self.color.disable();
    }
}

impl Material for LinesMaterial {
    fn render(
        &mut self,
        pass: usize,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        camera: &mut dyn Camera,
        _: &Light,
        data: &ObjectData,
        _mesh: &mut Mesh,
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

        let mut data = data
            .user_data()
            .downcast_ref::<Rc<RefCell<LinesData>>>()
            .unwrap()
            .borrow_mut();

        data.bind(&mut self.pos, &mut self.color);

        ctxt.enable(Context::BLEND);
        ctxt.blend_func_separate(
            Context::SRC_ALPHA,
            Context::ONE_MINUS_SRC_ALPHA,
            Context::ONE,
            Context::ONE_MINUS_SRC_ALPHA,
        );
        ctxt.line_width(self.lines_width);
        ctxt.draw_elements(
            Context::LINES,
            data.edges.len() as i32 * 2,
            Context::UNSIGNED_SHORT,
            0,
        );
        ctxt.line_width(1.0);
        ctxt.disable(Context::BLEND);

        data.unbind();

        self.deactivate();
    }
}

/// Vertex shader used by the material to display line.
const LINES_VERTEX_SRC: &str = "#version 100
    attribute vec3 position;
    attribute vec4 color;
    varying   vec4 frag_color;
    uniform   mat3 scale;
    uniform   mat4 proj, view, transform;
    void main() {
        gl_Position = proj * view * transform * vec4(position, 1.0);
        frag_color = color;
    }";

/// Fragment shader used by the material to display line.
const LINES_FRAGMENT_SRC: &str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec4 frag_color;
    void main() {
        gl_FragColor = frag_color;
    }";
