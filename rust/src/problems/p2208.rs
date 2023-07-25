pub struct Solution;

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq)]
struct Float(f64);

impl Eq for Float {}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut half = nums.iter().map(|&e| e as i64).sum::<i64>() as f64 / 2.0;
        let mut q: BinaryHeap<_> = nums.into_iter().map(|n| Float(n as f64)).collect();
        let mut result = 0;
        while half > 0.0 {
            let n = q.pop().unwrap().0 / 2.0;
            half -= n;
            q.push(Float(n));
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::halve_array(vec![5, 19, 8, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::halve_array(vec![3, 8, 20]));
    }
}
