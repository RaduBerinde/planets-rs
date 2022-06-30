use self::lighting::*;
use self::material::*;
use crate::{body::Body, system::System};
use kiss3d::camera::Camera;
use kiss3d::nalgebra;
use kiss3d::nalgebra::Point2;
use kiss3d::nalgebra::Vector2;
use kiss3d::{
    camera::ArcBall,
    event::MouseButton,
    nalgebra::{Point3, Translation3, Vector3},
    ncollide3d::procedural,
    resource::{Material, Mesh},
    scene::SceneNode,
    window::Window,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod lighting;
mod material;

pub struct Renderer<'a> {
    s: &'a mut System,

    camera: ArcBall,
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

        let mut camera = ArcBall::new_with_frustrum(
            std::f32::consts::PI / 4.0,
            0.001,
            10240.0,
            Point3::new(0.0f32, 0.0, 10.0),
            Point3::origin(),
        );
        camera.rebind_drag_button(Some(MouseButton::Button1));
        camera.rebind_rotate_button(Some(MouseButton::Button2));
        camera.set_dist_step(0.99);

        camera.set_at(render_position(&s.earth));

        Renderer { s, camera, bodies }
    }

    // Returns false if the window should be closed.
    pub fn frame(&mut self, window: &mut Window) -> bool {
        self.s.for_all(|body| {
            let pos = render_position(body);
            let translation = Translation3::new(pos.x, pos.y, pos.z);
            let render_state = self.bodies.get_mut(&body.name).unwrap();
            render_state.scene_node.set_local_translation(translation);

            if body.name != "sun" {
                body_lighting(body, &render_state.mesh, 2.0 * body.radius as f32);
            }
        });
        self.render_body_hint(window, &self.s.earth);
        self.render_body_hint(window, &self.s.moon);
        self.render_body_hint(window, &self.s.sun);

        window.render_with_camera(&mut self.camera)
    }

    pub fn render_body_hint(&self, window: &mut Window, body: &Body) {
        let body_pos = render_position(body);

        let dist = (body_pos - self.camera.eye()).norm();
        if dist < render_radius(body) * 200.0 {
            return;
        }

        let point = self.camera.project(&body_pos, &Vector2::new(1.0, 1.0));
        let mut win_size: Vector2<f32> = nalgebra::convert(window.size());
        win_size /= window.scale_factor() as f32;

        let point = &win_size.component_mul(&(point - Vector2::new(0.5, 0.5)));

        window.draw_planar_line(
            &Point2::new(point.x, point.y - 15.0),
            &Point2::new(point.x, point.y + 15.0),
            &body.color,
        );

        window.draw_planar_line(
            &Point2::new(point.x - 15.0, point.y),
            &Point2::new(point.x + 15.0, point.y),
            &body.color,
        );
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
