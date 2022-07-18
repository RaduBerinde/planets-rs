use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use kiss3d::{
    nalgebra::{self, Point2, Point3, Point4, Vector3},
    resource::{AllocationType, MaterialManager, Mesh},
    window::Window,
};

use super::lines_material::LinesData;

pub struct Trail {
    max_length: f64,
    max_points: usize,
    min_dist: f64,
    color: Point4<f32>,

    history: VecDeque<DataPoint>,

    lines_data: Rc<RefCell<LinesData>>,
}

struct DataPoint {
    p: Point3<f64>,
    dist_to_prev: f64,
}

impl Trail {
    pub fn new(
        window: &mut Window,
        max_length: f64,
        max_points: usize,
        color: Point4<f32>,
    ) -> Self {
        // We add an object with an empty mesh, then we associate it with the
        // lines data and material.
        let mut node = window.add_mesh(
            Rc::new(RefCell::new(Mesh::new(
                Vec::new(),
                Vec::new(),
                None,
                None,
                false,
            ))),
            Vector3::new(1.0, 1.0, 1.0),
        );
        node.set_material(MaterialManager::get_global_manager(|m| {
            m.get("lines").unwrap()
        }));
        let lines_data = Rc::new(RefCell::new(LinesData::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            AllocationType::DynamicDraw,
        )));
        node.data_mut()
            .get_object_mut()
            .set_user_data(Box::new(Rc::clone(&lines_data)));

        Self {
            max_length,
            max_points,
            min_dist: max_length / max_points as f64,
            color,
            history: VecDeque::new(),
            lines_data,
        }
    }

    pub fn reset(&mut self) {
        self.history.clear();
    }

    pub fn frame(&mut self, p: Point3<f64>, camera_focus: Point3<f64>) {
        if self.history.is_empty() {
            self.history.push_front(DataPoint {
                p,
                dist_to_prev: 0.0,
            });
            return;
        }

        let mut lines_data = self.lines_data.borrow_mut();
        let coords = lines_data.coords.data_mut().as_mut().unwrap();
        coords.clear();
        coords.push(nalgebra::convert(p - camera_focus.coords));
        for dp in &self.history {
            coords.push(nalgebra::convert(dp.p - camera_focus.coords));
        }

        let dist_to_prev = (p - self.history[0].p).norm();

        let dist_to_alpha_scale = 1.0 / self.max_length;

        let colors = lines_data.colors.data_mut().as_mut().unwrap();
        colors.clear();
        colors.push(self.color);
        let mut dist_so_far = dist_to_prev;
        for dp in &self.history {
            let mut color = self.color;
            color.w *= (1.0 - dist_so_far * dist_to_alpha_scale) as f32;
            colors.push(color);
            dist_so_far += dp.dist_to_prev;
        }

        let edges = lines_data.edges.data_mut().as_mut().unwrap();
        edges.clear();
        for i in 0..self.history.len() {
            edges.push(Point2::new(i as u16, (i + 1) as u16));
        }
        if dist_to_prev >= self.min_dist {
            if self.history.len() >= self.max_points {
                self.history.pop_back();
            }
            self.history.push_front(DataPoint { p, dist_to_prev });
        }
    }
}
