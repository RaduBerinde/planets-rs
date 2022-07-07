use std::time::{Duration, Instant};

use delegate::delegate;
use kiss3d::{
    camera::{ArcBall, Camera},
    event::WindowEvent,
    nalgebra::{Isometry3, Matrix4, Point3},
    resource::ShaderUniform,
    window::Canvas,
};

pub struct MyCamera {
    arcball: ArcBall,
    transition: Option<TransitionState>,
}

struct TransitionState {
    start_time: Instant,
    start_eye: Point3<f32>,
    start_focus: Point3<f32>,
    target_time: Instant,
    target_eye: Point3<f32>,
    target_focus: Point3<f32>,

    last_update_time: Instant,
}

impl MyCamera {
    pub fn new() -> Self {
        MyCamera {
            arcball: ArcBall::new_with_frustrum(
                std::f32::consts::PI / 4.0,
                0.001,
                10240.0,
                Point3::new(0.0, 0.0, 3000.0),
                Point3::origin(),
            ),
            transition: None,
        }
    }

    pub fn update_focus(&mut self, focus: Point3<f32>) {
        match self.transition.as_mut() {
            None => self.arcball.set_at(focus),

            Some(transition) => {
                transition.target_eye += focus - transition.target_focus;
                transition.target_focus = focus;

                let now = Instant::now();
                let time_left = transition.target_time - now;
                if time_left.is_zero() {
                    self.arcball
                        .look_at(transition.target_eye, transition.target_focus);
                    self.transition = None;
                    return;
                }

                // Interpolate exponentially.
                let t = (now - transition.last_update_time).as_secs_f32()
                    / (transition.target_time - transition.last_update_time).as_secs_f32();
                let t = 1.0 - (0.003_f32).powf(t);
                let focus = self.arcball.at();
                let eye = self.arcball.eye();
                self.arcball.look_at(
                    eye + (transition.target_eye - eye) * t,
                    focus + (transition.target_focus - focus) * t,
                );
                transition.last_update_time = now;
            }
        }
    }

    const TRANSITION_TIME: Duration = Duration::from_nanos(250_000_000);

    pub fn transition_to(&mut self, eye: Point3<f32>, focus: Point3<f32>) {
        let now = Instant::now();
        self.transition = Some(TransitionState {
            start_time: now,
            start_eye: self.arcball.eye(),
            start_focus: self.arcball.at(),
            target_time: now + Self::TRANSITION_TIME,
            target_eye: eye,
            target_focus: focus,
            last_update_time: now,
        })
    }

    const SCROLL_STEP: f32 = 0.99;

    pub fn handle_scroll(&mut self, off: f32) {
        self.arcball
            .set_dist(self.arcball.dist() * Self::SCROLL_STEP.powf(off));
    }
}

impl Camera for MyCamera {
    delegate! {
        to self.arcball {
            fn eye(&self) -> Point3<f32>;
            fn view_transform(&self) -> Isometry3<f32>;
            fn transformation(&self) -> Matrix4<f32>;
            fn inverse_transformation(&self) -> Matrix4<f32>;
            fn clip_planes(&self) -> (f32, f32); // fixme: should this be here?
            fn update(&mut self, canvas: &Canvas);
            fn upload(
                &self,
                pass: usize,
                proj: &mut ShaderUniform<Matrix4<f32>>,
                view: &mut ShaderUniform<Matrix4<f32>>,
            );
        }
    }
    fn handle_event(&mut self, canvas: &Canvas, event: &WindowEvent) {
        match *event {
            WindowEvent::Scroll(_, off, _) => self.handle_scroll(off as f32),
            _ => self.arcball.handle_event(canvas, event),
        }
    }
}
