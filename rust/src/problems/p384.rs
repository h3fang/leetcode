use rand::prelude::*;

pub struct Solution {
    nums: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self {
            nums,
            rng: thread_rng(),
        }
    }

    pub fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    pub fn shuffle(&mut self) -> Vec<i32> {
        let mut r = self.nums.clone();
        for i in 0..r.len() {
            let k = self.rng.gen_range(i..r.len());
            r.swap(i, k);
        }
        r
    }
}
