use std::{
    f64::consts::{PI, TAU},
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

use super::interpolate;

pub struct MyCamera {
    projection: Perspective3<f64>,
    // 2D Displacement of projection on X axis; used to move the center to
    // account for the UI panel.
    dx_px: f64,
    proj: Matrix4<f32>,
    view: Matrix4<f32>,
    proj_view: Matrix4<f32>,
    // The camera maintains the focus point, but it does not actually apply it.
    // Instead, objects are expected to be translated by the camera focus, and
    // the camera always points at the origin. This is to prevent f32
    // instability when the camera is close to an object that is far from the
    // origin.
    focus: Point3<f64>,
    // The camera eye, in relation to the focus, is located
    // at [0, 0, +dist] before the pitch and yaw rotations are applied.
    dist: f64,
    min_dist: f64,
    max_dist: f64,
    // The pitch and yaw are not affected by mouse control; see user_yaw, user_pitch.
    // pitch = 0 is a top down view; pitch = PI/2 is a side view.
    pitch: f64,
    // yaw = 0 is looking towards the y axis.
    yaw: f64,

    // Change in yaw and pitch due to user input.
    user_pitch: f64,
    user_yaw: f64,

    min_pitch: f64,
    max_pitch: f64,

    dist_scale_next_frame: Option<f64>,
    last_cursor_pos: Vector2<f64>,
    last_framebuffer_size: Vector2<u32>,

    transition: Option<TransitionState>,
}

struct TransitionState {
    target_time: Instant,
    target_focus: Point3<f64>,
    target_pitch: f64,
    target_yaw: f64,
    target_dist: f64,
    mid_dist: f64,
    min_dist_after_transition: f64,

    to_intermediate: interpolate::Params,
    from_intermediate: interpolate::Params,
    focus_interp: interpolate::Params,

    start_time: Instant,
    last_t: f64,
}

impl MyCamera {
    const TRANSITION_TIME: Duration = Duration::from_millis(500);
    const TRANSITION_SIGMOID_K: f64 = 5.0;
    const OVERHEAD_DIST: f64 = 5e+8;

    pub fn new(dx_px: f64) -> Self {
        let fov = std::f64::consts::PI / 4.0;
        let aspect = 800.0 / 600.0;
        let (znear, zfar) = (5e+2, 2e+10);

        let mut res = Self {
            projection: Perspective3::new(aspect, fov, znear, zfar),
            dx_px,
            proj: nalgebra::zero(),
            view: nalgebra::zero(),
            proj_view: nalgebra::zero(),
            focus: Point3::new(0.0, 0.0, 0.0),
            dist: Self::OVERHEAD_DIST,
            min_dist: 1e+4,
            max_dist: Self::OVERHEAD_DIST,
            pitch: 0.0,
            yaw: 0.0,
            user_pitch: 0.0,
            user_yaw: 0.0,
            min_pitch: 0.0,
            max_pitch: PI * 0.75,
            dist_scale_next_frame: None,
            last_cursor_pos: nalgebra::zero(),
            last_framebuffer_size: Vector2::new(800, 600),
            transition: None,
        };
        res.calc_matrices();
        res
    }

    pub fn update(&mut self, focus: Point3<f64>, eye_dir: Vector3<f64>) {
        let (pitch, yaw) = Self::pitch_and_yaw(eye_dir);
        if let Some(scale) = self.dist_scale_next_frame {
            self.dist = (self.dist * scale).clamp(self.min_dist, self.max_dist);
            self.dist_scale_next_frame = None;
            self.calc_matrices();
        }

        match self.transition.as_mut() {
            None => {
                self.focus = focus;
                if self.pitch != pitch || self.yaw != yaw {
                    self.pitch = pitch;
                    self.yaw = yaw;
                    self.enforce_pitch_limits();
                    self.calc_matrices();
                }
            }

            Some(transition) => {
                transition.target_focus = focus;

                let now = Instant::now();
                if transition.target_time.duration_since(now).is_zero() {
                    self.focus = transition.target_focus;
                    self.dist = transition.target_dist;
                    self.pitch = transition.target_pitch;
                    self.yaw = transition.target_yaw;
                    self.min_dist = transition.min_dist_after_transition;
                    self.transition = None;
                } else {
                    let last_t = transition.last_t;
                    let t = (now - transition.start_time).as_secs_f64()
                        / (transition.target_time - transition.start_time).as_secs_f64();
                    assert!(last_t <= t);

                    if t < transition.to_intermediate.t_end {
                        self.dist = transition.to_intermediate.interpolate(
                            transition.mid_dist,
                            self.dist,
                            last_t,
                            t,
                        );
                        self.pitch = transition.to_intermediate.interpolate(
                            0.0,
                            norm_radian(self.pitch),
                            last_t,
                            t,
                        );
                        self.yaw = transition.to_intermediate.interpolate(
                            0.0,
                            norm_radian(self.yaw),
                            last_t,
                            t,
                        );
                    } else {
                        self.dist = transition.from_intermediate.interpolate(
                            transition.target_dist,
                            self.dist,
                            last_t,
                            t,
                        );
                        self.pitch = transition.from_intermediate.interpolate(
                            self.pitch + norm_radian(transition.target_pitch - self.pitch),
                            self.pitch,
                            last_t,
                            t,
                        );
                        self.yaw = transition.from_intermediate.interpolate(
                            self.yaw + norm_radian(transition.target_yaw - self.yaw),
                            self.yaw,
                            last_t,
                            t,
                        );
                    }
                    self.focus = transition.focus_interp.interpolate(
                        transition.target_focus,
                        self.focus,
                        last_t,
                        t,
                    );
                    transition.last_t = t;
                }
                self.calc_matrices();
            }
        }
    }

    pub fn transition_to(
        &mut self,
        focus: Point3<f64>,
        eye_dir: Vector3<f64>,
        dist: f64,
        min_dist: f64,
    ) {
        // Calculate a distance from which both bodies would be visible (in their current positions).
        // We will first zoom out to that distance.
        let mut mid_dist = 4.0 * (self.focus - focus).norm();
        mid_dist = mid_dist.max(self.dist).max(dist);
        let mut t_mid = 0.0;
        if mid_dist > self.dist {
            t_mid = (mid_dist - self.dist) / (2.0 * mid_dist - self.dist - dist);
        }

        // Commit the current user angle changes. Note that further changes are
        // disabled during the transition.
        self.pitch += self.user_pitch;
        self.user_pitch = 0.0;
        self.yaw += self.user_yaw;
        self.user_yaw = 0.0;

        let (target_pitch, target_yaw) = Self::pitch_and_yaw(eye_dir);

        let k = Self::TRANSITION_SIGMOID_K;
        let now = Instant::now();
        self.transition = Some(TransitionState {
            target_time: now + Self::TRANSITION_TIME,
            target_focus: focus,
            target_pitch,
            target_yaw,
            target_dist: dist,
            mid_dist,
            min_dist_after_transition: min_dist as f64,
            start_time: now,
            last_t: 0.0,

            to_intermediate: interpolate::Params::new(k, 0.0, t_mid),
            from_intermediate: interpolate::Params::new(k, t_mid, 1.0),
            focus_interp: interpolate::Params::new(k, t_mid * 0.7, 1.0),
        })
    }

    pub fn focus(&self) -> Point3<f64> {
        self.focus
    }

    pub fn dist(&self) -> f32 {
        self.dist as f32
    }

    fn rotation(&self) -> UnitQuaternion<f64> {
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -self.yaw - self.user_yaw)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch + self.user_pitch)
    }

    fn calc_matrices(&mut self) {
        let mut proj = *self.projection.as_matrix();
        let dx = f64::clamp(self.dx_px / self.last_framebuffer_size.x as f64, -0.4, 0.4);
        proj = Translation3::new(dx, 0.0, 0.0).to_homogeneous() * proj;

        let view = self.view_transform_64().to_homogeneous();
        let proj_view = proj * view;

        self.proj = nalgebra::convert(proj);
        self.view = nalgebra::convert(view);
        self.proj_view = nalgebra::convert(proj_view);
        //self.inverse_proj_view = self.proj_view.try_inverse().unwrap();
    }

    pub fn view_transform_64(&self) -> Isometry3<f64> {
        Isometry3::from_parts(
            Translation3::new(0.0, 0.0, -self.dist),
            self.rotation().inverse(),
        )
    }

    const SCROLL_STEP: f64 = 0.99;
    const YAW_STEP: f64 = 0.005;
    const PITCH_STEP: f64 = 0.005;

    fn handle_scroll(&mut self, off: f32) {
        if self.transition.is_none() {
            self.dist_scale_next_frame = Some(Self::SCROLL_STEP.powf(off as f64));
        }
    }

    fn handle_rotation(&mut self, dpos: Vector2<f64>) {
        if self.transition.is_some() {
            return;
        }
        self.user_yaw += dpos.x * Self::YAW_STEP;
        self.user_pitch -= dpos.y * Self::PITCH_STEP;
        self.enforce_pitch_limits();
        self.calc_matrices();
    }

    fn enforce_pitch_limits(&mut self) {
        let actual_pitch = norm_radian(self.pitch + self.user_pitch);
        if actual_pitch > self.max_pitch {
            self.user_pitch = norm_radian(self.max_pitch - self.pitch);
        } else if actual_pitch < self.min_pitch {
            self.user_pitch = norm_radian(self.min_pitch - self.pitch);
        }
    }

    fn pitch_and_yaw(eye_dir: Vector3<f64>) -> (f64, f64) {
        if eye_dir.x == 0.0 && eye_dir.y == 0.0 && eye_dir.z > 0.0 {
            // Special case for top-down views.
            return (0.0, 0.0);
        }
        let yaw = -0.5 * PI - f64::atan2(eye_dir.y, eye_dir.x);
        let pitch = eye_dir.angle(&Vector3::new(0.0, 0.0, 1.0));
        (pitch, yaw)
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

                self.last_cursor_pos = curr_pos;
            }

            WindowEvent::FramebufferSize(w, h) => {
                let vec = Vector2::new(w, h);
                if self.last_framebuffer_size != vec {
                    self.last_framebuffer_size = vec;
                    self.projection.set_aspect(w as f64 / h as f64);
                    self.calc_matrices();
                }
            }

            _ => {}
        }
    }

    // eye is the camera eye, relative to the focus.
    fn eye(&self) -> Point3<f32> {
        nalgebra::convert(self.rotation() * Point3::new(0.0, 0.0, self.dist))
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
        (
            self.projection.znear() as f32,
            self.projection.zfar() as f32,
        )
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

// norm_radian normalizes the given angle to the range [-PI, PI].
// Useful when interpolating between two angles - the norm_radian of the
// difference is the "shortest way around the circle".
fn norm_radian(angle: f64) -> f64 {
    let a = angle.rem_euclid(TAU);
    if a > PI {
        a - TAU
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_radian() {
        let tc = [
            (0.0, 0.0),
            (1.0, 1.0),
            (-1.0, -1.0),
            (PI + 1.0, -PI + 1.0),
            (PI - 1.0, PI - 1.0),
            (-PI + 1.0, -PI + 1.0),
            (-PI - 1.0, PI - 1.0),
        ];
        for (a, expected) in tc {
            for delta in [0.0, TAU, -TAU, 2.0 * TAU, -3.0 * TAU] {
                let actual = norm_radian(a + delta);
                assert!((actual - expected).abs() < 1e-5);
            }
        }
    }
}
