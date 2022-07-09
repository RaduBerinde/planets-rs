use kiss3d::{
    nalgebra::{Point3, Vector3},
    window::Window,
};

pub struct Grid {
    step: f32,
    num_steps: i32,
    axis_color: Point3<f32>,
    color: Point3<f32>,
}

impl Grid {
    pub fn new(step: f32, num_steps: i32) -> Self {
        Self {
            step,
            num_steps,
            axis_color: Point3::new(0.4, 0.4, 0.1),
            color: Point3::new(0.2, 0.3, 0.4),
        }
    }

    pub fn render(&mut self, window: &mut Window, origin: Point3<f32>, scale: f32) {
        let step = self.step * scale;
        let size = step * self.num_steps as f32;
        for i in -self.num_steps..=self.num_steps {
            let d = i as f32 * step;
            let color = if i == 0 { self.axis_color } else { self.color };
            window.draw_line(
                &(origin + Vector3::new(d, -size, 0.0)),
                &(origin + Vector3::new(d, size, 0.0)),
                &color,
            );

            window.draw_line(
                &(origin + Vector3::new(-size, d, 0.0)),
                &(origin + Vector3::new(size, d, 0.0)),
                &color,
            );
        }
    }
}
