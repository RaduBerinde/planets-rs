use self::body_material::*;
use self::camera::*;
use self::grid::Grid;
use self::lines_material::LinesMaterial;
use self::trail::Trail;
use self::ui::Ui;
use crate::body::Body;
use crate::body::Body::*;
use crate::choice::Choice;
use crate::choice::ChoiceSet;
use crate::control::CameraDirection;
use crate::control::CameraSpec;
use crate::control::ControlEvent;
use crate::render::flat_material::FlatMaterial;
use crate::render::skybox::Skybox;
use crate::simulation::Snapshot;
use crate::state::RenderState;
use crate::state::SimulationState;
use kiss3d::camera::Camera;

use kiss3d::light::Light;
use kiss3d::nalgebra;
use kiss3d::nalgebra::Isometry3;
use kiss3d::nalgebra::Point2;
use kiss3d::nalgebra::Point4;
use kiss3d::nalgebra::UnitQuaternion;
use kiss3d::nalgebra::Vector2;
use kiss3d::nalgebra::Vector3;
use kiss3d::resource::MaterialManager;
use kiss3d::resource::Texture;
use kiss3d::resource::TextureManager;

use kiss3d::{
    nalgebra::{Point3, Translation3},
    resource::Material,
    scene::SceneNode,
    window::Window,
};
use std::path::Path;

use std::{cell::RefCell, rc::Rc};

mod body_material;
mod camera;
mod flat_material;
mod grid;
mod interpolate;
mod lines_material;
mod skybox;
mod trail;
mod ui;

pub struct Renderer {
    camera: MyCamera,

    camera_focus: Choice<CameraSpec>,

    grid: Grid,
    skybox: Skybox,

    sun_node: SceneNode,

    earth_node: SceneNode,
    earth_lighting: Rc<RefCell<BodyLightingData>>,
    earth_day_texture: Rc<Texture>,
    earth_night_texture: Rc<Texture>,
    earth_day_blurred_texture: Rc<Texture>,
    earth_night_blurred_texture: Rc<Texture>,
    earth_axis: Option<SceneNode>,
    earth_trail: Trail,

    moon_node: SceneNode,
    moon_lighting: Rc<RefCell<BodyLightingData>>,
    moon_trail: Trail,

    ui: Ui,

    snapshot: Snapshot,
}

const SHOW_EARTH_AXIS: bool = true;

impl Renderer {
    pub fn new(snapshot: &Snapshot, window: &mut Window) -> Self {
        window.set_light(Light::StickToCamera);
        window.set_line_width(2.0);

        // Init materials.
        MaterialManager::get_global_manager(|m| {
            m.add(
                Rc::new(RefCell::new(
                    Box::new(BodyMaterial::new()) as Box<dyn Material + 'static>
                )),
                "body",
            );
            m.add(
                Rc::new(RefCell::new(
                    Box::new(LinesMaterial::new()) as Box<dyn Material + 'static>
                )),
                "lines",
            );
            m.add(
                Rc::new(RefCell::new(
                    Box::new(FlatMaterial::new()) as Box<dyn Material + 'static>
                )),
                "flat",
            );
        });

        // Init the Sun. The sun uses the default material.
        let mut sun_node = window.add_sphere(Sun.radius());
        sun_node.set_color(1.5, 1.5, 1.5);
        println!("Loading sun texture");
        sun_node.set_texture_from_file(Path::new("./media/sun.jpg"), "sun");

        // Init the Earth. The earth uses our custom body material.
        let mut earth_node = window.add_sphere(Earth.radius());
        earth_node.set_material(MaterialManager::get_global_manager(|m| {
            m.get("body").unwrap()
        }));

        println!("Loading earth textures");
        let earth_day_texture = TextureManager::get_global_manager(|tm| {
            tm.add(Path::new("./media/2k_earth_daymap.jpg"), "earth-day")
        });
        let earth_night_texture = TextureManager::get_global_manager(|tm| {
            tm.add(Path::new("./media/2k_earth_nightmap.jpg"), "earth-night")
        });
        let earth_day_blurred_texture = TextureManager::get_global_manager(|tm| {
            tm.add(
                Path::new("./media/2k_earth_daymap_blurred.jpg"),
                "earth-day-blurred",
            )
        });
        let earth_night_blurred_texture = TextureManager::get_global_manager(|tm| {
            tm.add(
                Path::new("./media/2k_earth_nightmap_blurred.jpg"),
                "earth-night-blurred",
            )
        });
        let earth_lighting = Rc::new(RefCell::new(BodyLightingData {
            day_color: Point3::new(1.3, 1.3, 1.3),
            day_texture: None, // will be set each frame.
            night_color: Point3::new(0.9, 0.9, 0.9),
            night_texture: None, // will be set each frame.
            normal_texture: Some(TextureManager::get_global_manager(|tm| {
                tm.add(Path::new("./media/earth_normal.png"), "earth-normal-map")
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
                    window.add_cylinder(Earth.radius() * 0.01, Earth.radius() * 3.0);
                scene_node.set_color(0.5, 0.5, 0.05);
                Some(scene_node)
            }
        };

        let earth_trail = Trail::new(
            window,
            2e9, // Earth orbit length is about 1e9
            1000,
            Point4::new(
                Earth.props().color.0,
                Earth.props().color.1,
                Earth.props().color.2,
                0.6,
            ),
        );

        // Init the Moon. The moon also uses our custom body material.
        let mut moon_node = window.add_sphere(Moon.radius());
        moon_node.set_material(MaterialManager::get_global_manager(|m| {
            m.get("body").unwrap()
        }));

        println!("Loading moon textures");
        let moon_lighting = Rc::new(RefCell::new(BodyLightingData {
            day_color: Point3::new(1.1, 1.1, 1.1),
            day_texture: Some(TextureManager::get_global_manager(|tm| {
                tm.add(Path::new("./media/2k_moon.jpg"), "moon")
            })),
            //day_texture: Some(TextureManager::get_global_manager(|tm| tm.get_default())),
            night_color: Moon.props().color_vec() * 0.1,
            night_texture: Some(TextureManager::get_global_manager(|tm| tm.get_default())),
            //normal_texture: None,
            normal_texture: Some(TextureManager::get_global_manager(|tm| {
                tm.add(Path::new("./media/moon_normal.jpg"), "moon-normal-map")
            })),
            ..BodyLightingData::default()
        }));
        moon_node
            .data_mut()
            .get_object_mut()
            .set_user_data(Box::new(Rc::clone(&moon_lighting)));

        let moon_trail = Trail::new(
            window,
            1e9, // Earth orbit length is about 1e9
            1000,
            Point4::new(
                Moon.props().color.0,
                Moon.props().color.1,
                Moon.props().color.2,
                0.6,
            ),
        );

        let camera = MyCamera::new(-Ui::WIDTH * window.scale_factor());
        let skybox = Skybox::new(window, 2e+10);
        let grid = Grid::new(window, 20);
        let ui = Ui::new(window);

        let camera_specs = [
            CameraSpec {
                focus: Earth,
                direction: CameraDirection::FromAbove,
                dist: 10.0 * Earth.radius64(),
                description: "Earth",
            },
            CameraSpec {
                focus: Moon,
                direction: CameraDirection::FromAbove,
                dist: 30.0 * Moon.radius64(),
                description: "Moon",
            },
            CameraSpec {
                focus: Moon,
                direction: CameraDirection::FromBody(Earth),
                dist: 10.0 * Moon.radius64(),
                description: "Moon phase",
            },
            CameraSpec {
                focus: Sun,
                direction: CameraDirection::FromAbove,
                dist: 10.0 * Sun.radius64(),
                description: "Sun",
            },
        ];

        let mut renderer = Renderer {
            camera,
            camera_focus: ChoiceSet::new(camera_specs).by_index(0),
            grid,
            skybox,
            sun_node,
            earth_node,
            earth_lighting,
            earth_day_texture,
            earth_night_texture,
            earth_day_blurred_texture,
            earth_night_blurred_texture,
            earth_axis,
            earth_trail,
            moon_node,
            moon_lighting,
            moon_trail,
            ui,
            snapshot: *snapshot,
        };

        renderer.transition_camera(renderer.camera_focus.get());

        println!("Rendering initialized");

        renderer
    }

    pub fn set_snapshot(&mut self, snapshot: &Snapshot) {
        self.snapshot = *snapshot
    }

    // Returns false if the window should be closed.
    pub fn frame(
        &mut self,
        window: &mut Window,
        sim_state: &dyn SimulationState,
    ) -> Vec<ControlEvent> {
        let cam_spec = self.camera_focus.get();
        let focus_pos = self.abs_position(cam_spec.focus);
        let eye_vec = match cam_spec.direction {
            CameraDirection::FromAbove => Vector3::z_axis().into_inner(),
            CameraDirection::FromBody(b) => self.abs_position(b) - focus_pos,
        };
        self.camera.update(focus_pos, eye_vec);

        self.grid.update(
            Point3::new(0.0, 0.0, -self.camera.focus().z as f32),
            (self.camera.dist() + self.camera.focus().z as f32) * 4.0,
        );

        // Sun.
        self.sun_node
            .set_local_transformation(self.transformation(Sun));

        // Earth.
        let earth_transformation = self.transformation(Earth);
        self.earth_node
            .set_local_transformation(earth_transformation);

        if self.earth_axis.is_some() {
            self.earth_axis
                .as_mut()
                .unwrap()
                .set_local_transformation(earth_transformation);
        }

        {
            let mut earth_lighting = self.earth_lighting.borrow_mut();

            let blur_earth = sim_state.is_running() && sim_state.speed().num_days() > 5;
            if !blur_earth {
                earth_lighting.day_texture = Some(Rc::clone(&self.earth_day_texture));
                earth_lighting.night_texture = Some(Rc::clone(&self.earth_night_texture));
            } else {
                earth_lighting.day_texture = Some(Rc::clone(&self.earth_day_blurred_texture));
                earth_lighting.night_texture = Some(Rc::clone(&self.earth_night_blurred_texture));
            }
            earth_lighting.light_pos = self.render_position(Sun);
            earth_lighting.light_radius = Sun.radius();
            earth_lighting.occluder_pos = self.render_position(Moon);
            earth_lighting.occluder_radius = Moon.radius();
        }

        self.earth_trail
            .frame(self.abs_position(Earth), self.camera.focus());

        // Moon.
        self.moon_node
            .set_local_transformation(self.transformation(Moon));

        {
            let mut moon_lighting = self.moon_lighting.borrow_mut();
            moon_lighting.light_pos = self.render_position(Sun);
            moon_lighting.light_radius = Sun.radius();
            moon_lighting.occluder_pos = self.render_position(Earth);
            moon_lighting.occluder_radius = Earth.radius();
        }
        self.moon_trail
            .frame(self.abs_position(Moon), self.camera.focus());

        for body in [Sun, Earth, Moon] {
            self.render_body_hint(&self.camera, window, body);
        }

        let mut events = self.ui.frame(window, sim_state, &*self);
        if !window.render_with_camera(&mut self.camera) {
            return vec![ControlEvent::Exit];
        }
        for mut event in window.events().iter() {
            if let Some(ev) = ControlEvent::from_window_event(&mut event) {
                events.push(ev);
            }
        }
        events
    }

    fn render_body_hint(&self, camera: &MyCamera, window: &mut Window, body: Body) {
        let body_pos = self.render_position(body);

        // Only show the hint if we see the object as very small.
        let dist = (body_pos - camera.eye()).norm();
        if dist < body.radius() * 200.0 {
            return;
        }

        let projected =
            Point3::from_homogeneous(camera.transformation() * body_pos.to_homogeneous()).unwrap();

        if projected.z > 1.0 {
            // Object behind us.
            return;
        }

        let scale = 0.5 / window.scale_factor() as f32;
        let point = Point2::new(
            projected.x * window.width() as f32 * scale,
            projected.y * window.height() as f32 * scale,
        );

        let color = Point3::new(
            body.props().color.0,
            body.props().color.1,
            body.props().color.2,
        );

        let mut draw_line = |angle: f32| {
            const START: f32 = 6.0;
            const END: f32 = 14.0;
            let rad = angle.to_radians();
            let axis = Vector2::new(rad.cos(), rad.sin());

            window.draw_planar_line(&(point + axis * START), &(point + axis * END), &color);
        };

        for a in [0, 90, 180, 270] {
            // Earth gets the vertical cross-hairs; Moon gets the diagonal; Sun
            // gets both.
            if body == Sun || body == Earth {
                draw_line(a as f32);
            }
            if body == Sun || body == Moon {
                draw_line((a + 45) as f32);
            }
        }
    }

    fn transition_camera(&mut self, spec: CameraSpec) {
        let body = spec.focus;
        let focus = self.abs_position(body);
        let radius = body.radius64();
        self.camera.transition_to(focus, spec.dist, radius * 1.5);
    }

    pub fn handle_event(&mut self, event: &ControlEvent) {
        match event {
            ControlEvent::CycleCamera => {
                self.camera_focus = self.camera_focus.circular_next();
                self.transition_camera(self.camera_focus.get());
            }
            ControlEvent::SetCamera(camera_focus) => {
                self.camera_focus = camera_focus.clone();
                self.transition_camera(self.camera_focus.get());
            }
            ControlEvent::Reverse => {
                self.earth_trail.reset();
                self.moon_trail.reset();
            }
            ControlEvent::ToggleTrails => {
                let visible = !self.earth_trail.is_visible();
                self.earth_trail.set_visible(visible);
                self.moon_trail.set_visible(visible);
            }
            ControlEvent::ToggleEcliptic => {
                self.grid.set_visible(!self.grid.is_visible());
            }
            ControlEvent::ToggleSkybox => {
                self.skybox.set_visible(!self.skybox.is_visible());
            }
            _ => {}
        }
    }

    pub fn render_position(&self, body: Body) -> Point3<f32> {
        nalgebra::convert(self.abs_position(body) - self.camera.focus().coords)
    }

    pub fn abs_position(&self, body: Body) -> Point3<f64> {
        match body {
            Sun => Point3::default(),
            Earth => self.snapshot.earth_position,
            Moon => self.snapshot.moon_position,
        }
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

impl RenderState for Renderer {
    fn camera_focus(&self) -> Choice<CameraSpec> {
        self.camera_focus.clone()
    }

    fn show_trails(&self) -> bool {
        self.earth_trail.is_visible()
    }

    fn show_ecliptic(&self) -> bool {
        self.grid.is_visible()
    }

    fn show_skybox(&self) -> bool {
        self.skybox.is_visible()
    }
}
