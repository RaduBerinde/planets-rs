use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub struct Fps {
    frames: VecDeque<Instant>,
    fps: f64,
}

impl Fps {
    const MIN_FRAMES: usize = 2;
    const MAX_FRAMES: usize = 100;
    const TIMEFRAME: Duration = Duration::from_secs(1);

    pub fn new() -> Fps {
        Fps {
            frames: VecDeque::with_capacity(Self::MAX_FRAMES),
            fps: 0.0,
        }
    }

    pub fn frame(&mut self) {
        let now = Instant::now();
        if self.frames.len() < Self::MIN_FRAMES {
            self.frames.push_back(now);
            return;
        }
        let cutoff = now - Self::TIMEFRAME;
        let f = &mut self.frames;
        while f.len() >= Self::MAX_FRAMES
            || (f.len() >= Self::MIN_FRAMES && *f.front().unwrap() < cutoff)
        {
            f.pop_front();
        }
        f.push_back(now);
        self.fps = (f.len() - 1) as f64 / (now - *f.front().unwrap()).as_secs_f64();
    }

    pub fn get(&self) -> f64 {
        self.fps
    }
}
