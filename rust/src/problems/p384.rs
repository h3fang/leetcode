// use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    pub fn shuffle(&self) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut r = self.nums.clone();
        // r.shuffle(&mut rng);
        for i in 0..r.len() {
            let k = rng.gen_range(i..r.len());
            r.swap(i, k);
        }
        r
    }
}
