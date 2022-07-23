use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Seconds(pub f64);

impl Seconds {
    pub fn at_least(self, other: Self) -> Self {
        Seconds(self.0.max(other.0))
    }
    pub fn at_most(self, other: Self) -> Self {
        Seconds(self.0.min(other.0))
    }

    pub fn to_duration(self) -> chrono::Duration {
        chrono::Duration::nanoseconds((self.0 * 1e9) as i64)
    }
}

impl From<chrono::Duration> for Seconds {
    fn from(duration: chrono::Duration) -> Self {
        Seconds(duration.num_nanoseconds().unwrap() as f64 * 1e-9)
    }
}

impl From<std::time::Duration> for Seconds {
    fn from(duration: std::time::Duration) -> Self {
        Seconds(duration.as_secs_f64())
    }
}

impl Mul<f64> for Seconds {
    type Output = Seconds;
    fn mul(self, other: f64) -> Seconds {
        Seconds(self.0 * other)
    }
}

impl Div<f64> for Seconds {
    type Output = Seconds;
    fn div(self, other: f64) -> Seconds {
        Seconds(self.0 / other)
    }
}

impl Div<Seconds> for Seconds {
    type Output = f64;
    fn div(self, other: Seconds) -> f64 {
        self.0 / other.0
    }
}
