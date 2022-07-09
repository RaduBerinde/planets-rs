use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Isometry3, Matrix3, Matrix4, Point2, Point3, Point4, Vector3};
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
}

// LinesData is used as for object's generic data.
pub struct LinesData {
    coords: Rc<RefCell<GPUVec<Point3<f32>>>>,
    colors: Rc<RefCell<GPUVec<Point4<f32>>>>,
    edges: Rc<RefCell<GPUVec<Point2<u16>>>>,
}

impl LinesData {
    pub fn new(
        coords: Vec<Point3<f32>>,
        colors: Vec<Point4<f32>>,
        edges: Vec<Point2<u16>>,
    ) -> Self {
        let location = AllocationType::StaticDraw;
        LinesData {
            coords: Rc::new(RefCell::new(GPUVec::new(
                coords,
                BufferType::Array,
                location,
            ))),
            colors: Rc::new(RefCell::new(GPUVec::new(
                colors,
                BufferType::Array,
                location,
            ))),
            edges: Rc::new(RefCell::new(GPUVec::new(
                edges,
                BufferType::ElementArray,
                location,
            ))),
        }
    }

    pub fn bind_coords(&self, coords: &mut ShaderAttribute<Point3<f32>>) {
        coords.bind(&mut *self.coords.borrow_mut());
    }

    pub fn bind_colors(&self, colors: &mut ShaderAttribute<Point4<f32>>) {
        colors.bind(&mut *self.colors.borrow_mut());
    }

    pub fn bind_edges(&self) {
        self.edges.borrow_mut().bind();
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

    varying vec3 frag_color;
    void main() {
        gl_FragColor = frag_color;
    }";
