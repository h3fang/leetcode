pub struct Solution;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let n = barcodes.len();
        let mut m = HashMap::new();
        for c in barcodes {
            *m.entry(c).or_insert(0) += 1;
        }
        let mut q = m
            .into_iter()
            .map(|(x, cx)| (cx, x))
            .collect::<BinaryHeap<_>>();
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let (cx, x) = q.pop().unwrap();
            if i == 0 || result[i - 1] != x {
                result.push(x);
                if cx > 1 {
                    q.push((cx - 1, x));
                }
            } else {
                let (cy, y) = q.pop().unwrap();
                result.push(y);
                q.push((cx, x));
                q.push((cy - 1, y));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut result: Vec<i32>, mut nums: Vec<i32>) {
        assert!(result.windows(2).all(|w| w[0] != w[1]));
        nums.sort_unstable();
        result.sort_unstable();
        assert_eq!(nums, result);
    }

    #[test]
    fn case1() {
        let nums = vec![1, 1, 1, 2, 2, 2];
        check(Solution::rearrange_barcodes(nums.to_vec()), nums);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 1, 1, 1, 2, 2, 3, 3];
        check(Solution::rearrange_barcodes(nums.to_vec()), nums);
    }
}
