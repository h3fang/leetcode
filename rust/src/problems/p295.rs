use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
pub struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        if self.high.len() < self.low.len() {
            self.low.push(num);
            let n = self.low.pop().unwrap();
            self.high.push(Reverse(n));
        } else {
            self.high.push(Reverse(num));
            let n = self.high.pop().unwrap().0;
            self.low.push(n);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.low.len() > self.high.len() {
            *self.low.peek().unwrap() as f64
        } else {
            let a = self.low.peek().unwrap();
            let b = self.high.peek().unwrap().0;
            0.5 * (a + b) as f64
        }
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
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_close(1.5, mf.find_median());
        mf.add_num(3);
        assert_close(2.0, mf.find_median());
    }
}
