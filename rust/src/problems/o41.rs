use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        if self.left.len() > self.right.len() {
            self.left.push(num);
            let large = self.left.pop().unwrap();
            self.right.push(-large);
        } else {
            self.right.push(-num);
            let small = -self.right.pop().unwrap();
            self.left.push(small);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else {
            0.5 * (*self.left.peek().unwrap() as f64 - *self.right.peek().unwrap() as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert!((mf.find_median() - 1.5).abs() < f64::EPSILON);
        mf.add_num(3);
        assert!((mf.find_median() - 2.0).abs() < f64::EPSILON);
    }
}
