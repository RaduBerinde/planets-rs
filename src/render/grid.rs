use std::{cell::RefCell, rc::Rc};

use kiss3d::{
    nalgebra::{Point2, Point3, Point4, Translation3, Vector3},
    resource::{AllocationType, MaterialManager, Mesh},
    scene::SceneNode,
    window::Window,
};

use super::lines_material::{LinesData};

pub struct Grid {
    node: SceneNode,
}

impl Grid {
    pub fn new(window: &mut Window, num_steps: i32) -> Self {
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
        let axis_color = Point4::new(0.6, 0.6, 0.3, 0.5);
        let color = Point4::new(0.4, 0.5, 0.6, 0.5);

        node.set_material(MaterialManager::get_global_manager(|m| {
            m.get("lines").unwrap()
        }));

        let mut coords: Vec<Point3<f32>> = Vec::new();
        let mut colors: Vec<Point4<f32>> = Vec::new();
        let mut edges: Vec<Point2<u16>> = Vec::new();

        let step = 1.0 / num_steps as f32;
        let alpha_scale = |x: f32, y: f32, color: Point4<f32>| -> Point4<f32> {
            Point4::new(color.x, color.y, color.z, color.w * (1.0 - (x * x + y * y)))
        };
        for i in -num_steps..=num_steps {
            for j in -num_steps..=num_steps {
                let (x, y) = (i as f32 * step, j as f32 * step);
                coords.push(Point3::new(x, y, 0.0));
                colors.push(alpha_scale(x, y, color));
            }
        }
        let to_idx = |i: i32, j: i32| -> u16 {
            ((i + num_steps) * (2 * num_steps + 1) + (j + num_steps)) as u16
        };
        for i in -num_steps..=num_steps - 1 {
            for j in -num_steps..=num_steps - 1 {
                if j != 0 {
                    edges.push(Point2::new(to_idx(i, j), to_idx(i + 1, j)));
                }
                if i != 0 {
                    edges.push(Point2::new(to_idx(i, j), to_idx(i, j + 1)));
                }
            }
        }

        // Add X axis.
        let _idx_start = coords.len() as u16;
        for i in -num_steps..=num_steps {
            let x = i as f32 * step;
            coords.push(Point3::new(x, 0.0, 0.0));
            colors.push(alpha_scale(x, 0.0, axis_color));
            if i < num_steps {
                let idx = coords.len() as u16 - 1;
                edges.push(Point2::new(idx, idx + 1));
            }
        }

        // Add Y axis.
        for j in -num_steps..=num_steps {
            let y = j as f32 * step;
            coords.push(Point3::new(0.0, y, 0.0));
            colors.push(alpha_scale(0.0, y, axis_color));
            if j < num_steps {
                let idx = coords.len() as u16 - 1;
                edges.push(Point2::new(idx, idx + 1));
            }
        }

        //for i in -self.num_steps..=self.num_steps {
        //    for j in -self.num_steps..=self.num_steps {
        //        let d = i as f32 * step;
        //        let color = if i == 0 { self.axis_color } else { self.color };
        //        window.draw_line(
        //            &(origin + Vector3::new(d, -size, 0.0)),
        //            &(origin + Vector3::new(d, size, 0.0)),
        //            &color,
        //        );

        //        window.draw_line(
        //            &(origin + Vector3::new(-size, d, 0.0)),
        //            &(origin + Vector3::new(size, d, 0.0)),
        //            &color,
        //        );
        //    }
        //}

        node.data_mut()
            .get_object_mut()
            .set_user_data(Box::new(Rc::new(RefCell::new(LinesData::new(
                coords,
                colors,
                edges,
                AllocationType::StaticDraw,
            )))));

        Self { node }
    }

    pub fn update(&mut self, origin: Point3<f32>, scale: f32) {
        self.node.set_local_scale(scale, scale, scale);
        self.node
            .set_local_translation(Translation3::new(origin.x, origin.y, origin.z));
    }
}
