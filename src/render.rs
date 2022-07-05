use self::camera_mover::*;
use self::material::*;
use crate::body::Body;
use crate::body::Body::*;
use crate::simulate::*;
use chrono::{DateTime, TimeZone, Utc};
use kiss3d::camera::Camera;
use kiss3d::event::Action;
use kiss3d::event::Key;
use kiss3d::event::WindowEvent;
use kiss3d::light::Light;
use kiss3d::nalgebra;
use kiss3d::nalgebra::Point2;
use kiss3d::nalgebra::Vector2;
use kiss3d::text::Font;
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

mod camera_mover;
mod material;

pub struct Renderer {
    camera: ArcBall,
    camera_mover: CameraMover,
    tab_press_count: u32,

    sun_node: SceneNode,

    earth_node: SceneNode,
    earth_lighting: Rc<RefCell<BodyLightingData>>,

    moon_node: SceneNode,
    moon_lighting: Rc<RefCell<BodyLightingData>>,

    snapshot: Snapshot,
}

impl Renderer {
    pub fn new(window: &mut Window) -> Self {
        let snapshot = Snapshot::simple();

        let mut camera = ArcBall::new_with_frustrum(
            std::f32::consts::PI / 4.0,
            0.001,
            10240.0,
            Point3::new(0.0, 0.0, 8.0),
            Point3::origin(),
        );
        camera.rebind_drag_button(Some(MouseButton::Button1));
        camera.rebind_rotate_button(Some(MouseButton::Button2));
        camera.rebind_reset_key(None);
        camera.set_dist_step(0.99);
        //camera.set_min_dist(1e-6);
        //camera.set_max_dist(1e+6);

        window.set_light(Light::StickToCamera);

        let mut sun_node = window.add_sphere(render_radius(Sun));
        sun_node.set_color(1.5, 1.5, 1.5);
        sun_node.set_texture_from_file(Path::new("./media/sun.jpg"), "sun");

        let mat = Rc::new(RefCell::new(
            Box::new(MyMaterial::new()) as Box<dyn Material + 'static>
        ));

        let mut init_body = |body: Body| -> (SceneNode, Rc<RefCell<BodyLightingData>>) {
            let mut node = window.add_sphere(render_radius(body));

            let color = body.props().color;
            node.set_color(color.0, color.1, color.2);
            node.set_material(Rc::clone(&mat));
            let lighting = Rc::new(RefCell::new(BodyLightingData::default()));

            node.data_mut()
                .get_object_mut()
                .set_user_data(Box::new(Rc::clone(&lighting)));

            (node, lighting)
        };

        let (earth_node, earth_lighting) = init_body(Earth);
        let (moon_node, moon_lighting) = init_body(Moon);

        let mut renderer = Renderer {
            camera,
            camera_mover: CameraMover::new(),
            tab_press_count: 0,
            sun_node,
            earth_node,
            earth_lighting,
            moon_node,
            moon_lighting,
            snapshot: snapshot,
        };

        renderer.camera.set_at(renderer.render_position(Earth));

        renderer
    }

    // Returns false if the window should be closed.
    pub fn frame(&mut self, window: &mut Window) -> bool {
        self.handle_events(window);
        self.camera_mover.maybe_move_camera(&mut self.camera);

        window.draw_text(
            &self.snapshot.timestamp.to_string(),
            &Point2::new(20.0, 10.0),
            100.0,
            &Font::default(),
            &Point3::new(0.8, 0.8, 0.8),
        );

        for body in [Sun, Earth, Moon] {
            let pos = self.render_position(body);
            let translation = Translation3::new(pos.x, pos.y, pos.z);
            let node = match body {
                Sun => &mut self.sun_node,
                Earth => &mut self.earth_node,
                Moon => &mut self.moon_node,
            };
            node.set_local_translation(translation);
            self.render_body_hint(&self.camera, window, body);
        }
        {
            let mut earth_lighting = self.earth_lighting.borrow_mut();

            earth_lighting.light_pos = self.render_position(Sun);
            earth_lighting.light_radius = render_radius(Sun);
            earth_lighting.occluder_pos = self.render_position(Moon);
            earth_lighting.occluder_radius = render_radius(Moon);
        }

        {
            let mut moon_lighting = self.moon_lighting.borrow_mut();
            moon_lighting.light_pos = self.render_position(Sun);
            moon_lighting.light_radius = render_radius(Sun);
            moon_lighting.occluder_pos = self.render_position(Earth);
            moon_lighting.occluder_radius = render_radius(Earth);
        }

        window.render_with_camera(&mut self.camera)
    }

    fn render_body_hint(&self, camera: &ArcBall, window: &mut Window, body: Body) {
        let body_pos = self.render_position(body);

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
        let color = Point3::new(
            body.props().color.0,
            body.props().color.1,
            body.props().color.2,
        );

        if body == Sun || body == Earth {
            const DELTA: f32 = 12.0;
            window.draw_planar_line(
                &Point2::new(point.x, point.y - DELTA),
                &Point2::new(point.x, point.y + DELTA),
                &color,
            );

            window.draw_planar_line(
                &Point2::new(point.x - DELTA, point.y),
                &Point2::new(point.x + DELTA, point.y),
                &color,
            );
        }
        if body == Sun || body == Moon {
            const DELTA: f32 = 8.5;
            window.draw_planar_line(
                &Point2::new(point.x - DELTA, point.y - DELTA),
                &Point2::new(point.x + DELTA, point.y + DELTA),
                &color,
            );

            window.draw_planar_line(
                &Point2::new(point.x - DELTA, point.y + DELTA),
                &Point2::new(point.x + DELTA, point.y - DELTA),
                &color,
            );
        }
    }

    fn handle_events(&mut self, window: &mut Window) {
        for mut event in window.events().iter() {
            match event.value {
                //WindowEvent::Scroll(xshift, yshift, modifiers) => {
                //    // dont override the default mouse handler
                //    event.value = WindowEvent::Scroll(xshift, -yshift * 0.3, modifiers);
                //}
                WindowEvent::Key(Key::Tab, Action::Press, _) => {
                    self.tab_press_count += 1;
                    match self.tab_press_count % 3 {
                        0 => self.camera_mover.move_to(self.render_position(Earth), 8.0),
                        1 => self.camera_mover.move_to(self.render_position(Moon), 2.0),
                        2 => self.camera_mover.move_to(self.render_position(Sun), 50.0),
                        _ => (),
                    }
                    event.inhibited = true;
                }
                _ => {}
            }
        }
    }
}

pub const RENDER_SCALE: f64 = 1e-5;

fn to_render_scale(d: f64) -> f32 {
    (d * RENDER_SCALE) as f32
}

pub fn render_radius(body: Body) -> f32 {
    to_render_scale(body.props().radius)
}

impl Renderer {
    pub fn render_position(&self, body: Body) -> Point3<f32> {
        let pos = match body {
            Sun => Point3::default(),
            Earth => self.snapshot.earth_position,
            Moon => self.snapshot.moon_position,
        };
        Point3::new(
            to_render_scale(pos.x),
            to_render_scale(pos.y),
            to_render_scale(pos.z),
        )
    }
}
