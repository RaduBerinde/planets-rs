use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use kiss3d::{
    nalgebra::{Point2, Point3, Point4, Vector3},
    resource::{AllocationType, Mesh},
    window::Window,
};

use super::lines_material::LinesData;

pub struct Trail {
    min_dist_sq: f32,
    max_points: usize,
    color: Point4<f32>,

    history: VecDeque<Point3<f32>>,

    lines_data: Rc<RefCell<LinesData>>,
}

impl Trail {
    fn new(window: &mut Window, min_dist: f32, max_points: usize, color: Point4<f32>) -> Self {
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
            min_dist_sq: min_dist * min_dist,
            max_points,
            color,
            history: VecDeque::new(),
            lines_data,
        }
    }

    fn maybe_add_point(&mut self, p: &Point3<f32>) {
        if self.history.is_empty() || (p - self.history[0]).norm_squared() >= self.min_dist_sq {
            if self.history.len() >= self.max_points {
                self.history.pop_front();
            }
            self.history.push_back(*p)
        }
    }

    pub fn frame(&mut self, p: &Point3<f32>) {
        if !self.history.is_empty() {
            let mut lines_data = self.lines_data.borrow_mut();
            let coords = lines_data.coords.data_mut().as_mut().unwrap();
            coords.clear();
            for a in &self.history {
                coords.push(*a);
            }
            coords.push(*p);

            let colors = lines_data.colors.data_mut().as_mut().unwrap();
            colors.clear();
            for i in 0..=self.history.len() {
                colors.push(self.color)
            }

            let edges = lines_data.edges.data_mut().as_mut().unwrap();
            edges.clear();
            for i in 0..self.history.len() {
                edges.push(Point2::new(i as u16, (i + 1) as u16));
            }
        }
        self.maybe_add_point(p);
    }
}
