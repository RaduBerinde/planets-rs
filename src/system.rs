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
}
