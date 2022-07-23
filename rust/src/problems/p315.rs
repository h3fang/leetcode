pub struct Solution;

use std::collections::{BTreeSet, HashMap};

struct Bit {
    a: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self { a: vec![0; n] }
    }

    fn low_bit(x: i32) -> i32 {
        x & (-x)
    }

    fn update(&mut self, mut i: usize, v: i32) {
        while i < self.a.len() {
            self.a[i] += v;
            i += Self::low_bit(i as i32) as usize;
        }
    }

    fn query(&mut self, mut x: i32) -> i32 {
        let mut result = 0;
        while x > 0 {
            result += self.a[x as usize];
            x -= Self::low_bit(x);
        }
        result
    }
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let s = nums.iter().cloned().collect::<BTreeSet<_>>();
        let m = s
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i + 1))
            .collect::<HashMap<_, _>>();
        let mut result = vec![0; nums.len()];
        let mut a = Bit::new(m.len() + 1);
        for (i, n) in nums.iter().enumerate().rev() {
            let r = *m.get(n).unwrap() as i32;
            result[i] = a.query(r - 1);
            a.update(r as usize, 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 1, 1, 0], Solution::count_smaller(vec![5, 2, 6, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    }
}
