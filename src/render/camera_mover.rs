use std::time::{Duration, Instant};

use kiss3d::{camera::ArcBall, nalgebra::Point3};

pub struct CameraMover {
    target_time: Option<Instant>,
    target_at: Point3<f32>,
    target_dist: f32,
    last_time: Instant,
}

const CAMERA_TRANSITION_TIME: Duration = Duration::from_nanos(250_000_000);

impl CameraMover {
    pub fn new() -> Self {
        CameraMover {
            last_time: Instant::now(),
            target_time: None,
            target_at: Point3::origin(),
            target_dist: 0.0,
        }
    }

    pub fn move_to_with_transition_time(
        &mut self,
        new_at: Point3<f32>,
        new_dist: f32,
        transition_time: Duration,
    ) {
        let now = Instant::now();
        self.last_time = now;
        self.target_time = Some(now + transition_time);
        self.target_at = new_at;
        self.target_dist = new_dist;
    }

    pub fn move_to(&mut self, new_at: Point3<f32>, new_dist: f32) {
        self.move_to_with_transition_time(new_at, new_dist, CAMERA_TRANSITION_TIME);
    }

    pub fn maybe_move_camera(&mut self, camera: &mut ArcBall) {
        if let Some(target_time) = self.target_time {
            let now = Instant::now();

            let time_left = target_time - now;
            if time_left.is_zero() {
                camera.set_at(self.target_at);
                camera.set_dist(self.target_dist);
                self.target_time = None;
                return;
            }

            // Interpolate exponentially.
            let t =
                (now - self.last_time).as_secs_f32() / (target_time - self.last_time).as_secs_f32();
            let t = 1.0 - (0.003_f32).powf(t);
            let at = camera.at();
            camera.set_at(at + (self.target_at - at) * t);
            let dist = camera.dist();
            camera.set_dist(dist + (self.target_dist - dist) * t);
            self.last_time = now;
        }
    }
}
