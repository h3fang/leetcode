use std::collections::HashMap;

use rand::prelude::*;

const K: i32 = 30;

pub struct MajorityChecker {
    arr: Vec<i32>,
    p: HashMap<i32, Vec<i32>>,
    rng: ThreadRng,
}

fn lower_bound(nums: &[i32], target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l as i32
}

fn upper_bound(nums: &[i32], target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r as i32
}

impl MajorityChecker {
    pub fn new(arr: Vec<i32>) -> Self {
        let mut p: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &a) in arr.iter().enumerate() {
            p.entry(a).or_default().push(i as i32);
        }

        Self {
            arr,
            p,
            rng: thread_rng(),
        }
    }

    pub fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        let len = (right - left + 1) as usize;
        for _ in 0..K {
            let i = self.rng.gen_range(left..=right);
            let x = self.arr[i as usize];
            let pos = self.p.get(&x).unwrap();
            let ub = upper_bound(pos, right);
            let lb = lower_bound(pos, left);
            let n = ub - lb;
            if n >= threshold {
                return x;
            } else if n * 2 >= len as i32 {
                return -1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![1, 1, 2, 2, 1, 1];
        let mut mc = MajorityChecker::new(arr);
        assert_eq!(1, mc.query(0, 5, 4));
        assert_eq!(-1, mc.query(0, 3, 3));
        assert_eq!(2, mc.query(2, 3, 2));
    }
}
