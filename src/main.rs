use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3};
use kiss3d::window::Window;

use rand::random;

//fn main() {
//    let mut window = Window::new("Kiss3d: cube");
//    let mut c = window.add_cube(1.0, 1.0, 1.0);
//
//    c.set_color(1.0, 0.0, 0.0);
//
//    window.set_light(Light::StickToCamera);
//
//    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
//
//    while window.render() {
//        c.prepend_to_local_rotation(&rot);
//    }
//}

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

    c.append_translation(&Translation3::new(2.0, 0.0, 0.0));
    s.append_translation(&Translation3::new(4.0, 0.0, 0.0));
    p.append_translation(&Translation3::new(-2.0, 0.0, 0.0));
    y.append_translation(&Translation3::new(-4.0, 0.0, 0.0));
    a.append_translation(&Translation3::new(0.0, 0.0, 0.0));

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);

    while window.render() {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Scroll(xshift, yshift, modifiers) => {
                    // dont override the default mouse handler
                    event.value = WindowEvent::Scroll(xshift, -yshift * 0.3, modifiers);
                }
                _ => {}
            }
        }
        c.append_rotation_wrt_center(&rot);
        s.append_rotation_wrt_center(&rot);
        p.append_rotation_wrt_center(&rot);
        y.append_rotation_wrt_center(&rot);
        a.append_rotation_wrt_center(&rot);
    }
}
//use kiss3d::event::{Action, WindowEvent};
//use kiss3d::window::Window;
//
//fn main() {
//    let mut window = Window::new("Kiss3d: events");
//
//    while window.render() {
//        for mut event in window.events().iter() {
//            match event.value {
//                WindowEvent::Key(button, Action::Press, _) => {
//                    println!("You pressed the button: {:?}", button);
//                    println!("Do not try to press escape: the event is inhibited!");
//                    event.inhibited = true // override the default keyboard handler
//                }
//                WindowEvent::Key(button, Action::Release, _) => {
//                    println!("You released the button: {:?}", button);
//                    println!("Do not try to press escape: the event is inhibited!");
//                    event.inhibited = true // override the default keyboard handler
//                }
//                WindowEvent::MouseButton(button, Action::Press, mods) => {
//                    println!("You pressed the mouse button: {:?}", button);
//                    println!("You pressed the mouse button with modifiers: {:?}", mods);
//                    // dont override the default mouse handler
//                }
//                WindowEvent::MouseButton(button, Action::Release, mods) => {
//                    println!("You released the mouse button: {:?}", button);
//                    println!("You released the mouse button with modifiers: {:?}", mods);
//                    // dont override the default mouse handler
//                }
//                WindowEvent::CursorPos(x, y, _) => {
//                    println!("Cursor pos: ({} , {})", x, y);
//                    // dont override the default mouse handler
//                }
//                WindowEvent::Scroll(xshift, yshift, _) => {
//                    println!("Scroll: ({} , {})", xshift, yshift);
//                    // dont override the default mouse handler
//                }
//                _ => {}
//            }
//        }
//    }
//}
//
