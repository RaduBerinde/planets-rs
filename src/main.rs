mod body;
mod lighting;
mod material;
mod render;
mod system;

use crate::render::*;
use crate::system::*;

use kiss3d::camera::ArcBall;
use kiss3d::event::MouseButton;
use kiss3d::nalgebra::Point3;

use kiss3d::window::Window;

fn main() {
    let mut window = Window::new_with_size("planets-rs", 1200, 800);

    let mut camera = ArcBall::new_with_frustrum(
        std::f32::consts::PI / 4.0,
        0.001,
        10240.0,
        Point3::new(0.0f32, 0.0, 10.0),
        Point3::origin(),
    );
    camera.rebind_drag_button(Some(MouseButton::Button1));
    camera.rebind_rotate_button(Some(MouseButton::Button2));
    camera.set_dist_step(0.99);

    let mut s = System::new();

    const APHELION: f64 = 152.10e6;
    s.earth.position.x = APHELION;

    // Distance between earth and moon during Aug 21, 2017 solar eclipse.
    const MOON_TO_EARTH: f64 = 372000.0;
    s.moon.position.x = s.earth.position.x - MOON_TO_EARTH;

    camera.set_at(render_position(&s.earth));

    let mut r = Renderer::new(&mut s, &mut window);

    while r.frame(&mut window) {
        //for mut event in window.events().iter() {
        //    match event.value {
        //        WindowEvent::Scroll(xshift, yshift, modifiers) => {
        //            // dont override the default mouse handler
        //            event.value = WindowEvent::Scroll(xshift, -yshift * 0.3, modifiers);
        //        }
        //        _ => {}
        //    }
        //}
    }
}
