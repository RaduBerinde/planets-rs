use self::material::*;
use crate::{body::Body, system::System};
use kiss3d::camera::Camera;
use kiss3d::light::Light;
use kiss3d::nalgebra;
use kiss3d::nalgebra::Point2;
use kiss3d::nalgebra::Vector2;
use kiss3d::{
    camera::ArcBall,
    event::MouseButton,
    nalgebra::{Point3, Translation3},
    resource::Material,
    scene::SceneNode,
    window::Window,
};
use std::path::Path;
use std::{cell::RefCell, rc::Rc};

mod material;

pub struct Renderer<'a> {
    s: &'a mut System,

    camera: ArcBall,

    sun_node: SceneNode,

    earth_node: SceneNode,
    earth_lighting: Rc<RefCell<BodyLightingData>>,

    moon_node: SceneNode,
    moon_lighting: Rc<RefCell<BodyLightingData>>,
}

impl<'a> Renderer<'a> {
    pub fn new(s: &'a mut System, window: &mut Window) -> Self {
        let mut camera = ArcBall::new_with_frustrum(
            std::f32::consts::PI / 4.0,
            0.001,
            10240.0,
            Point3::new(0.0f32, 0.0, 10.0),
            Point3::origin(),
        );
        camera.rebind_drag_button(Some(MouseButton::Button1));
        camera.rebind_rotate_button(Some(MouseButton::Button2));
        camera.rebind_reset_key(None);
        camera.set_dist_step(0.99);
        //camera.set_min_dist(1e-6);
        //camera.set_max_dist(1e+6);

        camera.set_at(render_position(&s.earth));

        window.set_light(Light::StickToCamera);

        let mut sun_node = window.add_sphere(render_radius(&s.sun));
        sun_node.set_color(1.5, 1.5, 1.5);
        sun_node.set_texture_from_file(Path::new("./media/sun.jpg"), "sun");

        let mat = Rc::new(RefCell::new(
            Box::new(MyMaterial::new()) as Box<dyn Material + 'static>
        ));

        let mut init_body = |body: &Body| -> (SceneNode, Rc<RefCell<BodyLightingData>>) {
            let mut node = window.add_sphere(render_radius(body));

            node.set_color(body.color.x, body.color.y, body.color.z);
            node.set_material(Rc::clone(&mat));
            let lighting = Rc::new(RefCell::new(BodyLightingData::default()));

            node.data_mut()
                .get_object_mut()
                .set_user_data(Box::new(Rc::clone(&lighting)));

            (node, lighting)
        };

        let (earth_node, earth_lighting) = init_body(&s.earth);
        let (moon_node, moon_lighting) = init_body(&s.moon);

        Renderer {
            s,
            camera,
            sun_node,
            earth_node,
            earth_lighting,
            moon_node,
            moon_lighting,
        }
    }

    // Returns false if the window should be closed.
    pub fn frame(&mut self, window: &mut Window) -> bool {
        for (body, node) in [
            (&self.s.sun, &mut self.sun_node),
            (&self.s.earth, &mut self.earth_node),
            (&self.s.moon, &mut self.moon_node),
        ] {
            let pos = render_position(body);
            let translation = Translation3::new(pos.x, pos.y, pos.z);
            node.set_local_translation(translation);
            Renderer::render_body_hint(&self.camera, window, body);
        }
        {
            let mut earth_lighting = self.earth_lighting.borrow_mut();

            earth_lighting.light_pos = render_position(&self.s.sun);
            earth_lighting.light_radius = render_radius(&self.s.sun);
            earth_lighting.occluder_pos = render_position(&self.s.moon);
            earth_lighting.occluder_radius = render_radius(&self.s.moon);
        }

        {
            let mut moon_lighting = self.moon_lighting.borrow_mut();
            moon_lighting.light_pos = render_position(&self.s.sun);
            moon_lighting.light_radius = render_radius(&self.s.sun);
            moon_lighting.occluder_pos = render_position(&self.s.earth);
            moon_lighting.occluder_radius = render_radius(&self.s.earth);
        }

        window.render_with_camera(&mut self.camera)
    }

    fn render_body_hint(camera: &ArcBall, window: &mut Window, body: &Body) {
        let body_pos = render_position(body);

        // Only show the hint if we see the object as very small.
        let dist = (body_pos - camera.eye()).norm();
        if dist < render_radius(body) * 200.0 {
            return;
        }

        let projected =
            Point3::from_homogeneous(camera.transformation() * body_pos.to_homogeneous()).unwrap();

        if projected.z > 1.0 {
            // Object behind us.
            return;
        }

        let scale = nalgebra::convert::<_, Vector2<f32>>(window.size())
            * (0.5 / window.scale_factor() as f32);
        let point = projected.coords.xy().component_mul(&scale);

        if body.name == "sun" || body.name == "earth" {
            const DELTA: f32 = 12.0;
            window.draw_planar_line(
                &Point2::new(point.x, point.y - DELTA),
                &Point2::new(point.x, point.y + DELTA),
                &body.color,
            );

            window.draw_planar_line(
                &Point2::new(point.x - DELTA, point.y),
                &Point2::new(point.x + DELTA, point.y),
                &body.color,
            );
        }
        if body.name == "sun" || body.name == "moon" {
            const DELTA: f32 = 8.5;
            window.draw_planar_line(
                &Point2::new(point.x - DELTA, point.y - DELTA),
                &Point2::new(point.x + DELTA, point.y + DELTA),
                &body.color,
            );

            window.draw_planar_line(
                &Point2::new(point.x - DELTA, point.y + DELTA),
                &Point2::new(point.x + DELTA, point.y - DELTA),
                &body.color,
            );
        }
    }
}

//struct BodyRenderState {
//    scene_node: SceneNode,
//    mesh: Rc<RefCell<Mesh>>,
//}
//
//impl BodyRenderState {
//    pub fn new(
//        body: &Body,
//        mat: &Rc<RefCell<Box<dyn Material + 'static>>>,
//        window: &mut Window,
//    ) -> Self {
//        let mesh = Mesh::from_trimesh(procedural::unit_sphere(50, 50, true), false);
//        let mesh = Rc::new(RefCell::new(mesh));
//        let mut scene_node = window.add_mesh(
//            Rc::clone(&mesh),
//            Vector3::new(1.0, 1.0, 1.0) * (2.0 * render_radius(body)),
//        );
//        if body.name == "sun" {
//            scene_node.set_color(1.5, 1.5, 1.5);
//            scene_node.set_texture_from_file(Path::new("./media/sun.jpg"), "sun");
//        } else {
//            scene_node.set_color(body.color.x, body.color.y, body.color.z);
//            scene_node.set_material(Rc::clone(mat));
//        }
//
//        BodyRenderState { scene_node, mesh }
//    }
//}

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
