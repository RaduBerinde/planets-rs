use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use kiss3d::{
    camera::Camera,
    event::{Action, MouseButton, WindowEvent},
    nalgebra::{
        self, Isometry3, Matrix4, Perspective3, Point3, Translation3, UnitQuaternion, Vector2,
        Vector3,
    },
    resource::ShaderUniform,
    window::Canvas,
};

pub struct MyCamera {
    projection: Perspective3<f32>,
    proj: Matrix4<f32>,
    view: Matrix4<f32>,
    proj_view: Matrix4<f32>,
    //proj_view: Matrix4<f32>,
    focus: Point3<f64>,
    // The camera eye, in relation to the focus, is located
    // at [0, 0, +dist] with the rotation applied;
    dist: f64,
    min_dist: f64,
    max_dist: f64,
    yaw: f64,
    pitch: f64,
    min_pitch: f64,
    max_pitch: f64,

    dist_scale_next_frame: Option<f64>,
    last_cursor_pos: Vector2<f64>,

    transition: Option<TransitionState>,
}

struct TransitionState {
    target_time: Instant,
    target_focus: Point3<f64>,
    target_dist: f64,

    last_update_time: Instant,
}

impl MyCamera {
    pub fn new() -> Self {
        let fov = std::f32::consts::PI / 4.0;
        let aspect = 800.0 / 600.0;
        let (znear, zfar) = (1e+2, 1e+10);

        let mut res = Self {
            projection: Perspective3::new(aspect, fov, znear, zfar),
            proj: nalgebra::zero(),
            view: nalgebra::zero(),
            proj_view: nalgebra::zero(),
            focus: Point3::new(0.0, 0.0, 0.0),
            dist: 1e+8,
            min_dist: 1e+4,
            max_dist: 1e+9,
            yaw: 0.0,
            pitch: 0.0,
            min_pitch: 0.0,
            max_pitch: PI * 0.5,
            dist_scale_next_frame: None,
            last_cursor_pos: nalgebra::zero(),
            transition: None,
        };
        res.calc_matrices();
        res
    }

    pub fn update_focus(&mut self, focus: Point3<f64>) {
        if let Some(scale) = self.dist_scale_next_frame {
            self.dist = (self.dist * scale).clamp(self.min_dist, self.max_dist);
            self.dist_scale_next_frame = None;
        }

        match self.transition.as_mut() {
            None => self.focus = focus,

            Some(transition) => {
                transition.target_focus = focus;

                let now = Instant::now();
                if transition.target_time.duration_since(now).is_zero() {
                    self.focus = transition.target_focus;
                    self.dist = transition.target_dist;
                    self.pitch = 0.0;
                    self.yaw = 0.0;
                    self.transition = None;
                    return;
                }

                // Interpolate exponentially.
                let t = (now - transition.last_update_time).as_secs_f64()
                    / (transition.target_time - transition.last_update_time).as_secs_f64();
                let t = 1.0 - (0.003_f64).powf(t);
                self.focus += (transition.target_focus - self.focus) * t;
                self.dist += (transition.target_dist - self.dist) * t;
                self.pitch -= self.pitch * t;
                self.yaw -= self.yaw * t;
                transition.last_update_time = now;
            }
        }

        self.calc_matrices();
    }

    const TRANSITION_TIME: Duration = Duration::from_nanos(250_000_000);

    pub fn transition_to(&mut self, focus: Point3<f64>, dist: f64, min_dist: f64) {
        self.min_dist = min_dist as f64;

        let now = Instant::now();
        self.transition = Some(TransitionState {
            target_time: now + Self::TRANSITION_TIME,
            target_focus: focus,
            target_dist: dist,
            last_update_time: now,
        })
    }

    pub fn focus(&self) -> Point3<f32> {
        nalgebra::convert(self.focus)
    }

    pub fn dist(&self) -> f32 {
        self.dist as f32
    }

    pub fn eye_64(&self) -> Point3<f64> {
        let relative = self.rotation() * Point3::new(0.0, 0.0, self.dist);
        self.focus + relative.coords
    }

    fn rotation(&self) -> UnitQuaternion<f64> {
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -self.yaw)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch)
    }

    fn calc_matrices(&mut self) {
        self.proj = *self.projection.as_matrix();
        self.view = nalgebra::convert(self.view_transform_64().to_homogeneous());
        self.proj_view = self.proj * self.view;
        //self.inverse_proj_view = self.proj_view.try_inverse().unwrap();
    }

    pub fn view_transform_64(&self) -> Isometry3<f64> {
        let mut result = Isometry3::from_parts(Translation3::from(-self.focus), nalgebra::one());
        result.append_rotation_mut(&self.rotation().inverse());
        result.append_translation_mut(&Translation3::new(0.0, 0.0, -self.dist));

        result
    }

    const SCROLL_STEP: f64 = 0.99;
    const YAW_STEP: f64 = 0.005;
    const PITCH_STEP: f64 = 0.005;

    fn handle_scroll(&mut self, off: f32) {
        self.dist_scale_next_frame = Some(Self::SCROLL_STEP.powf(off as f64));
    }

    fn handle_rotation(&mut self, dpos: Vector2<f64>) {
        self.yaw += dpos.x * Self::YAW_STEP;
        while self.yaw < -PI {
            self.yaw += 2.0 * PI;
        }
        while self.yaw > PI {
            self.yaw -= 2.0 * PI;
        }
        self.yaw = self.yaw % PI;
        self.pitch -= dpos.y * Self::PITCH_STEP;
        self.pitch = self.pitch.clamp(self.min_pitch, self.max_pitch);
        self.calc_matrices();
    }
}

impl Camera for MyCamera {
    fn handle_event(&mut self, canvas: &Canvas, event: &WindowEvent) {
        match *event {
            WindowEvent::Scroll(_, off, _) => self.handle_scroll(off as f32),

            WindowEvent::CursorPos(x, y, _modifiers) => {
                let curr_pos = Vector2::new(x, y);

                if canvas.get_mouse_button(MouseButton::Button1) == Action::Press {
                    let dpos = curr_pos - self.last_cursor_pos;
                    self.handle_rotation(dpos)
                }

                //if let Some(drag_button) = self.drag_button {
                //    if canvas.get_mouse_button(drag_button) == Action::Press
                //        && self.drag_modifiers.map(|m| m == modifiers).unwrap_or(true)
                //    {
                //        let dpos = curr_pos - self.last_cursor_pos;
                //        let dpos_norm = dpos.component_div(&self.last_framebuffer_size);
                //        self.handle_right_button_displacement(&dpos_norm)
                //    }
                //}

                self.last_cursor_pos = curr_pos;
            }

            WindowEvent::FramebufferSize(w, h) => {
                self.projection.set_aspect(w as f32 / h as f32);
                self.calc_matrices();
            }

            _ => {}
        }
    }

    fn eye(&self) -> Point3<f32> {
        nalgebra::convert(self.eye_64())
    }

    fn view_transform(&self) -> Isometry3<f32> {
        nalgebra::convert(self.view_transform_64())
    }

    fn transformation(&self) -> Matrix4<f32> {
        self.proj_view
    }

    fn inverse_transformation(&self) -> Matrix4<f32> {
        unimplemented!()
    }

    fn clip_planes(&self) -> (f32, f32) {
        (self.projection.znear(), self.projection.zfar())
    }

    fn update(&mut self, _canvas: &Canvas) {}

    fn upload(
        &self,
        _pass: usize,
        proj: &mut ShaderUniform<Matrix4<f32>>,
        view: &mut ShaderUniform<Matrix4<f32>>,
    ) {
        proj.upload(&self.proj);
        view.upload(&self.view);
    }
}
