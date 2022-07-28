#[derive(Debug, Copy, Clone)]
pub struct Params {
    pub k: f64,
    pub t_start: f64,
    pub t_end: f64,
}

impl Params {
    pub fn new(k: f64, t_start: f64, t_end: f64) -> Self {
        Self { k, t_start, t_end }
    }

    // t_func is a sigmoid rescaling function for t. The larger k is, the slower
    // the change is around t_start and t_end (and the faster it is around the
    // middle).
    fn t_func(&self, t: f64) -> f64 {
        if t <= self.t_start {
            return 0.0;
        }
        if t >= self.t_end {
            return 1.0;
        }

        fn sigmoid(x: f64, k: f64) -> f64 {
            1.0 / (1.0 + f64::exp(-k * x))
        }

        // We use a sigmoid function, but we clip it to our range and rescale so
        // that 0 maps to 0 and 1 maps to 1.
        let low = sigmoid(-0.5, self.k);
        let high = sigmoid(0.5, self.k);
        (sigmoid(
            (t - self.t_start) / (self.t_end - self.t_start) - 0.5,
            self.k,
        ) - low)
            / (high - low)
    }

    pub fn interpolate<T, D>(&self, target: T, last: T, last_t: f64, t: f64) -> T
    where
        T: Copy + std::ops::Add<D, Output = T> + std::ops::Sub<T, Output = D>,
        D: std::ops::Mul<f64, Output = D>,
    {
        // We want to interpolate between the start value and the
        // end value. This is a bit tricky because the end value
        // (e.g. position) can be moving during the transition, so
        // we can't use the start value in the interpolation.
        //
        // t is in [0, 1] range.
        // We want to move slower at the beginning and end, so we
        // pass t through a function f(t).
        //
        // First, assume the start value is constant.
        // At t:
        //   pos(t) = start * (1 - f(t)) + end * f(t)
        //
        // At last (t-dt): let df = f(t) - f(t-dt)
        //   pos(t-dt) = start * (1 - f(t-dt)) + end * f(t-dt)
        //             = start * (1 - f(t) + df)) + end * (f(t) - df)
        //             = start * (1 - f(t)) + end * f(t) + df * (start - end)
        //             = pos(t) + df * (start - end)
        //
        //   pos(t) = pos(t-dt) + df * (end - start)
        //
        // Now let's remove the start value from the formula.
        //   start = (pos(t-dt) - end * f(t-dt)) / (1 - f(t-dt))
        //
        //   pos(t) = pos(t-dt) + df * (end - (pos(t-dt) - end * f(t-dt)) / (1 - f(t-dt)))
        //          = pos(t-dt) + df * (end * (1 - f(t-dt)) - pos(t-dt) + end * f(t-dt)) / (1 - f(t-dt)))
        //          = pos(t-dt) + df/(1 - f(t-dt)) * (end - pos(t-dt))
        let f = self.t_func(t);
        let last_f = self.t_func(last_t);
        let reldf = (f - last_f) / (1.0 - last_f);
        last + (target - last) * reldf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        datadriven::walk("src/render/testdata/interpolate", |f| {
            f.run(|tc| -> String {
                let k: f64 = tc.args.get("k").unwrap()[0].parse().unwrap();
                let mut t_start = 0.0;
                let mut t_end = 1.0;
                if let Some(val) = tc.args.get("t_range") {
                    t_start = val[0].parse().unwrap();
                    t_end = val[1].parse().unwrap();
                }
                let p = Params::new(k, t_start, t_end);

                match tc.directive.as_str() {
                    "t_func" => plot(|t| p.t_func(t)),
                    "interpolate" => {
                        let start: f64 = tc.args.get("start").unwrap()[0].parse().unwrap();
                        let target: f64 = tc.args.get("target").unwrap()[0].parse().unwrap();
                        let mut last = start;
                        let mut last_t: f64 = 0.0;
                        plot(|t| {
                            last = p.interpolate(target, last, last_t, t);
                            last_t = t;
                            last
                        })
                    }
                    _ => unimplemented!(),
                }
            })
        });
    }

    fn plot(mut f: impl FnMut(f64) -> f64) -> String {
        const ROWS: usize = 20;
        const COLS: usize = 60;

        let vals: Vec<f64> = (0..COLS)
            .map(|i| {
                let t = (i as f64) / (COLS - 1) as f64;
                f(t)
            })
            .collect();

        let min_val = vals
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let max_val = vals
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let mut plot = [['·'; COLS]; ROWS];
        const CH: char = '●';

        let mut last_row = 0;
        for i in 0..COLS {
            let scaled_val = (vals[i] - min_val) / (max_val - min_val);
            let row = (scaled_val * (ROWS - 1) as f64).round() as usize;
            if i > 0 {
                while last_row + 1 < row {
                    last_row += 1;
                    plot[last_row][i] = CH;
                }
                while last_row > row + 1 {
                    last_row -= 1;
                    plot[last_row][i] = CH;
                }
            }
            plot[row][i] = CH;
            last_row = row;
        }

        let y1 = format!("{:.1}", min_val);
        let y2 = format!("{:.1}", max_val);
        let width = usize::max(y1.len(), y2.len());

        let mut out = String::new();
        for i in 0..ROWS {
            use std::fmt::Write;
            let s: &str = match i + 1 {
                1 => &y2,
                ROWS => &y1,
                _ => "",
            };
            write!(out, "{:width$} ", s, width = width).unwrap();
            for j in 0..COLS {
                out.push(plot[ROWS - 1 - i][j])
            }
            out.push('\n');
        }

        out
    }
}
