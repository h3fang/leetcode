pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut sum: i64 = 0;
        let mut q = BinaryHeap::new();
        for (i, &e) in target.iter().enumerate() {
            sum += e as i64;
            q.push((e as i64, i));
        }
        while let Some((x, i)) = q.pop() {
            if x == 1 {
                break;
            }
            let r = sum - x;
            let a = if let Some(&(y, _)) = q.peek() {
                let k = if y == 1 {
                    (x - y + r - 1) / r
                } else {
                    (x - y) / r + 1
                };
                sum -= k * r;
                x - k * r
            } else {
                sum = x;
                x - r
            };
            if a < 1 {
                return false;
            }
            q.push((a, i));
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_possible(vec![9, 3, 5]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_possible(vec![1, 1, 1, 2]));
    }

    #[test]
    fn case3() {
        assert!(Solution::is_possible(vec![1, 10000000]));
    }
}
