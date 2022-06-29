use kiss3d::window::Window;

use super::body::Body;

pub struct System {
    pub sun: Body,
    pub earth: Body,
    pub moon: Body,
}

impl System {
    pub fn new() -> Self {
        System {
            sun: Body::sun(),
            earth: Body::earth(),
            moon: Body::moon(),
        }
    }

    pub fn render_init(&mut self, window: &mut Window) {
        self.for_all(|b| b.render_init(window));
    }

    pub fn render_update(&mut self) {
        self.for_all(|b| b.render_update());
    }

    fn for_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Body),
    {
        f(&mut self.sun);
        f(&mut self.earth);
        f(&mut self.moon);
    }
}
