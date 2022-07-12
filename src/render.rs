use self::camera::*;
use self::grid::Grid;
use self::shadow_material::*;
use crate::body::Body;
use crate::body::Body::*;
use crate::choice::Choice;
use crate::control::ControlEvent;
use crate::simulate::*;
use kiss3d::camera::Camera;

use kiss3d::light::Light;
use kiss3d::nalgebra;
use kiss3d::nalgebra::Isometry3;
use kiss3d::nalgebra::Point2;
use kiss3d::nalgebra::UnitQuaternion;
use kiss3d::nalgebra::Vector2;
use kiss3d::nalgebra::Vector3;
use kiss3d::resource::TextureManager;
use kiss3d::text::Font;
use kiss3d::{
    nalgebra::{Point3, Translation3},
    resource::Material,
    scene::SceneNode,
    window::Window,
};
use std::path::Path;

use std::{cell::RefCell, rc::Rc};

mod camera;
mod grid;
mod lines_material;
mod shadow_material;

pub struct Renderer {
    camera: MyCamera,
    camera_focus: Choice<Body>,

    grid: Grid,

    sun_node: SceneNode,

    earth_node: SceneNode,
    earth_lighting: Rc<RefCell<BodyLightingData>>,
    earth_axis: Option<SceneNode>,

    moon_node: SceneNode,
    moon_lighting: Rc<RefCell<BodyLightingData>>,

    snapshot: Snapshot,
}

const SHOW_EARTH_AXIS: bool = true;

impl Renderer {
    pub fn new(snapshot: &Snapshot, window: &mut Window) -> Self {
        //let mut camera = ArcBall::new_with_frustrum(
        //    std::f32::consts::PI / 4.0,
        //    0.001,
        //    10240.0,
        //    Point3::new(0.0, 0.0, 5000.0),
        //    Point3::origin(),
        //);
        //camera.rebind_drag_button(Some(MouseButton::Button1));
        //camera.rebind_rotate_button(Some(MouseButton::Button2));
        //camera.rebind_reset_key(None);
        //camera.set_dist_step(0.99);

        //camera.set_min_dist(1e-6);
        //camera.set_max_dist(1e+6);

        window.set_light(Light::StickToCamera);

        // Init the Sun. The sun uses the default material.
        let mut sun_node = window.add_sphere(render_radius(Sun));
        sun_node.set_color(1.5, 1.5, 1.5);
        sun_node.set_texture_from_file(Path::new("./media/sun.jpg"), "sun");

        let body_mat = Rc::new(RefCell::new(
            Box::new(ShadowMaterial::new()) as Box<dyn Material + 'static>
        ));

        // Init the Earth. The earth uses our custom shadow material.
        let mut earth_node = window.add_sphere(render_radius(Earth));
        earth_node.set_material(Rc::clone(&body_mat));

        let earth_lighting = Rc::new(RefCell::new(BodyLightingData {
            day_color: Point3::new(1.2, 1.2, 1.2),
            day_texture: Some(TextureManager::get_global_manager(|tm| {
                tm.add(Path::new("./media/2k_earth_daymap.jpg"), "earth-day")
            })),
            night_color: Point3::new(0.9, 0.9, 0.9),
            night_texture: Some(TextureManager::get_global_manager(|tm| {
                tm.add(Path::new("./media/2k_earth_nightmap.jpg"), "earth-night")
            })),
            ..BodyLightingData::default()
        }));
        earth_node
            .data_mut()
            .get_object_mut()
            .set_user_data(Box::new(Rc::clone(&earth_lighting)));

        let earth_axis = match SHOW_EARTH_AXIS {
            false => None,
            true => {
                let mut scene_node =
                    window.add_cylinder(render_radius(Earth) * 0.01, render_radius(Earth) * 3.0);
                scene_node.set_color(0.5, 0.5, 0.05);
                Some(scene_node)
            }
        };

        // Init the Moon. The moon also uses our custom shadow material.
        let mut moon_node = window.add_sphere(render_radius(Moon));
        moon_node.set_material(Rc::clone(&body_mat));

        let moon_lighting = Rc::new(RefCell::new(BodyLightingData {
            day_color: Point3::new(1.0, 1.0, 1.0),
            day_texture: Some(TextureManager::get_global_manager(|tm| {
                tm.add(Path::new("./media/2k_moon.jpg"), "moon")
            })),
            night_color: Moon.props().color_vec() * 0.1,
            night_texture: Some(TextureManager::get_global_manager(|tm| tm.get_default())),
            ..BodyLightingData::default()
        }));
        moon_node
            .data_mut()
            .get_object_mut()
            .set_user_data(Box::new(Rc::clone(&moon_lighting)));

        let camera = MyCamera::new();

        let mut renderer = Renderer {
            camera,
            camera_focus: Choice::new([Earth, Moon, Sun]),
            grid: Grid::new(window, 20),
            sun_node,
            earth_node,
            earth_lighting,
            earth_axis: earth_axis,
            moon_node,
            moon_lighting,
            snapshot: *snapshot,
        };

        renderer.transition_camera(renderer.camera_focus.get());

        renderer
    }

    pub fn set_snapshot(&mut self, snapshot: &Snapshot) {
        self.snapshot = *snapshot
    }

    // Returns false if the window should be closed.
    pub fn frame(&mut self, window: &mut Window) -> bool {
        self.camera
            .update_focus(self.render_position(self.camera_focus.get()));

        self.grid
            .update(self.camera.arcball.at(), self.camera.arcball.dist() * 4.0);

        window.draw_text(
            &self.snapshot.timestamp.to_string(),
            &Point2::new(20.0, 10.0),
            100.0,
            &Font::default(),
            &Point3::new(0.8, 0.8, 0.8),
        );

        /*
        let earth_rotation: UnitQuaternion<f32> = nalgebra::convert(
            self.snapshot.earth_orientation() *
            UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                -std::f64::consts::FRAC_PI_2,
            ),
        );
        let earth_transform = Isometry3::from_parts(
            Translation3::new(

        self.earth_node.set_local_transformation(nalgebra::one());
        // Reorient the Earth so that north points up.
        self.earth_node
            .set_local_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                -std::f32::consts::FRAC_PI_2,
            ));
        let earth_orientation: UnitQuaternion<f32> =
            nalgebra::convert(self.snapshot.earth_orientation());
        self.earth_node.append_rotation(&earth_orientation);
        //        self.earth_node
        //            .append_rotation(&UnitQuaternion::from_axis_angle(
        //                &Vector3::z_axis(),
        //                self.snapshot.earth_rotation_angle() as f32,
        //            ));
        if let Some(ref mut earth_axis) = self.earth_axis {
            earth_axis.set_local_transformation(self.earth_node.
        }
        */

        for body in [Sun, Earth, Moon] {
            let transformation = self.transformation(body);
            let node = match body {
                Sun => &mut self.sun_node,
                Earth => &mut self.earth_node,
                Moon => &mut self.moon_node,
            };
            node.set_local_transformation(transformation);
            self.render_body_hint(&self.camera, window, body);
        }

        if self.earth_axis.is_some() {
            let transformation = self.transformation(Earth);
            self.earth_axis
                .as_mut()
                .unwrap()
                .set_local_transformation(transformation);
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

    fn render_body_hint(&self, camera: &MyCamera, window: &mut Window, body: Body) {
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

    fn transition_camera(&mut self, body: Body) {
        let focus = self.render_position(body);
        let radius = to_render_scale(body.props().radius);
        let dist = radius
            * match body {
                Sun => 10.0,
                Earth => 10.0,
                Moon => 30.0,
            };
        self.camera
            .transition_to(focus + Vector3::new(0.0, 0.0, dist), focus, radius * 1.1);
    }

    pub fn handle_event(&mut self, event: ControlEvent) {
        match event {
            ControlEvent::CycleCamera => {
                self.camera_focus = self.camera_focus.circular_next();
                self.transition_camera(self.camera_focus.get());
            }
            _ => {}
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

    pub fn transformation(&self, body: Body) -> Isometry3<f32> {
        let pos = self.render_position(body);
        let translation = Translation3::new(pos.x, pos.y, pos.z);
        let rotation: UnitQuaternion<f32> = match body {
            Sun => nalgebra::one(),
            Moon => nalgebra::convert(
                self.snapshot.moon_orientation()
                    * UnitQuaternion::from_axis_angle(
                        &Vector3::x_axis(),
                        -std::f64::consts::FRAC_PI_2,
                    ),
            ),

            Earth => nalgebra::convert(
                self.snapshot.earth_orientation()
                    * UnitQuaternion::from_axis_angle(
                        &Vector3::x_axis(),
                        -std::f64::consts::FRAC_PI_2,
                    ),
            ),
        };
        Isometry3::from_parts(translation, rotation)
    }
}
