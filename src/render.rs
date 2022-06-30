use std::{cell::RefCell, collections::HashMap, rc::Rc};

use kiss3d::{
    nalgebra::{Point3, Translation3, Vector3},
    ncollide3d::procedural,
    resource::{Material, Mesh},
    scene::SceneNode,
    window::Window,
};

use crate::{
    body::Body,
    lighting::{body_lighting, init_sun_lighting},
    material::MyMaterial,
    system::System,
};

pub struct Renderer<'a> {
    s: &'a mut System,

    bodies: HashMap<String, Box<BodyRenderState>>,
}

impl<'a> Renderer<'a> {
    pub fn new(s: &'a mut System, window: &mut Window) -> Self {
        let mut bodies = HashMap::new();

        let mat = Rc::new(RefCell::new(
            Box::new(MyMaterial::new()) as Box<dyn Material + 'static>
        ));

        s.for_all(|body| {
            let render_state = BodyRenderState::new(body, &mat, window);

            bodies.insert(body.name.clone(), Box::new(render_state));
        });

        init_sun_lighting(&bodies.get("sun").unwrap().mesh);

        Renderer { s: s, bodies }
    }

    pub fn frame(&mut self) {
        self.s.for_all(|body| {
            let pos = render_position(body);
            let translation = Translation3::new(pos.x, pos.y, pos.z);
            let render_state = self.bodies.get_mut(&body.name).unwrap();
            render_state.scene_node.set_local_translation(translation);

            if body.name != "sun" {
                body_lighting(body, &render_state.mesh, 2.0 * body.radius as f32);
            }
        })
    }
}

struct BodyRenderState {
    scene_node: SceneNode,
    mesh: Rc<RefCell<Mesh>>,
}

impl BodyRenderState {
    pub fn new(
        body: &Body,
        mat: &Rc<RefCell<Box<dyn Material + 'static>>>,
        window: &mut Window,
    ) -> Self {
        let mesh = Mesh::from_trimesh(procedural::unit_sphere(50, 50, true), false);
        let mesh = Rc::new(RefCell::new(mesh));
        let mut scene_node = window.add_mesh(
            Rc::clone(&mesh),
            Vector3::new(1.0, 1.0, 1.0) * (2.0 * render_radius(body)),
        );
        scene_node.set_color(body.color.x, body.color.y, body.color.z);
        scene_node.set_material(Rc::clone(mat));

        BodyRenderState { scene_node, mesh }
    }
}

pub const RENDER_SCALE: f64 = 1e-5;

fn to_render_scale(d: f64) -> f32 {
    (d * RENDER_SCALE) as f32
}

pub fn render_radius(body: &Body) -> f32 {
    to_render_scale(body.radius)
}

pub fn render_position(body: &Body) -> Point3<f32> {
    Point3::new(
        to_render_scale(body.position.x),
        to_render_scale(body.position.y),
        to_render_scale(body.position.z),
    )
}
