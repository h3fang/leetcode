pub struct Solution {
    x_center: f64,
    y_center: f64,
    radius: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            x_center,
            y_center,
            radius,
        }
    }

    pub fn rand_point(&self) -> Vec<f64> {
        let theta = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        let u = rand::random::<f64>();
        let r = u.sqrt() * self.radius;
        let x = self.x_center + r * theta.cos();
        let y = self.y_center + r * theta.sin();
        vec![x, y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let r = 3.0;
        let x0 = 2.0;
        let y0 = 1.0;
        let s = Solution::new(r, x0, y0);
        let points = (0..20000).map(|_| s.rand_point()).collect::<Vec<_>>();
        let mut x_mean = 0.0;
        let mut y_mean = 0.0;
        for p in &points {
            let r2 = (p[0] - x0) * (p[0] - x0) + (p[1] - y0) * (p[1] - y0);
            assert!(r2 <= r * r);
            x_mean += p[0];
            y_mean += p[1];
        }
        x_mean /= points.len() as f64;
        y_mean /= points.len() as f64;
        assert!((x_mean - x0).abs() < 5e-2);
        assert!((y_mean - y0).abs() < 5e-2);
    }
}
