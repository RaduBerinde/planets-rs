use std::cell::RefCell;
use std::rc::Rc;

use crate::material::MyMaterial;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::resource::material::Material;

use kiss3d::window::Window;

use rand::random;

mod body;
mod material;

fn main() {
    let mut window = Window::new("Kiss3d: primitives");

    let mut c = window.add_cube(1.0, 1.0, 1.0);
    let mut s = window.add_sphere(0.5);
    let mut p = window.add_cone(0.5, 1.0);
    let mut y = window.add_cylinder(0.5, 1.0);
    let mut a = window.add_capsule(0.5, 1.0);

    c.set_color(random(), random(), random());
    s.set_color(random(), random(), random());
    p.set_color(random(), random(), random());
    y.set_color(random(), random(), random());
    a.set_color(random(), random(), random());
    let mat = Rc::new(RefCell::new(
        Box::new(MyMaterial::new()) as Box<dyn Material + 'static>
    ));

    c.set_material(Rc::clone(&mat));
    s.set_material(Rc::clone(&mat));
    p.set_material(Rc::clone(&mat));
    y.set_material(Rc::clone(&mat));
    a.set_material(Rc::clone(&mat));

    c.append_translation(&Translation3::new(2.0, 0.0, 0.0));
    s.append_translation(&Translation3::new(4.0, 0.0, 0.0));
    p.append_translation(&Translation3::new(-2.0, 0.0, 0.0));
    y.append_translation(&Translation3::new(-4.0, 0.0, 0.0));
    a.append_translation(&Translation3::new(0.0, 0.0, 0.0));

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);

    let mut camera = ArcBall::new(Point3::new(0.0f32, 0.0, -8.0), Point3::origin());
    camera.set_dist_step(-12.0);

    while window.render_with_camera(&mut camera) {
        //for mut event in window.events().iter() {
        //    match event.value {
        //        WindowEvent::Scroll(xshift, yshift, modifiers) => {
        //            // dont override the default mouse handler
        //            event.value = WindowEvent::Scroll(xshift, -yshift * 0.3, modifiers);
        //        }
        //        _ => {}
        //    }
        //}
        c.append_rotation_wrt_center(&rot);
        s.append_rotation_wrt_center(&rot);
        p.append_rotation_wrt_center(&rot);
        y.append_rotation_wrt_center(&rot);
        a.append_rotation_wrt_center(&rot);
    }
}
