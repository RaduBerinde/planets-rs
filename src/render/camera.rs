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
}

impl MyCamera {
    pub fn new() -> Self {
        MyCamera {
            arcball: ArcBall::new_with_frustrum(
                std::f32::consts::PI / 4.0,
                0.001,
                10240.0,
                Point3::new(0.0, 0.0, 5000.0),
                Point3::origin(),
            ),
        }
    }

    pub fn update_focus(&mut self, focus: Point3<f32>) {
        self.arcball.set_at(focus);
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
