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

    pub fn for_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Body),
    {
        f(&mut self.sun);
        f(&mut self.earth);
        f(&mut self.moon);
    }
}
