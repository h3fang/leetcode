use std::collections::VecDeque;

pub struct MovingAverage {
    s: usize,
    sum: i32,
    q: VecDeque<i32>,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        let s = size as usize;
        Self {
            s,
            sum: 0,
            q: VecDeque::with_capacity(s + 1),
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.q.push_back(val);
        self.sum += val;
        if self.q.len() > self.s {
            let v = self.q.pop_front().unwrap();
            self.sum -= v;
        }
        self.sum as f64 / self.q.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5);
    }

    #[test]
    fn case1() {
        let mut ma = MovingAverage::new(3);
        assert_close(1.0, ma.next(1));
        assert_close(5.5, ma.next(10));
        assert_close(4.66667, ma.next(3));
        assert_close(6.0, ma.next(5));
    }
}
