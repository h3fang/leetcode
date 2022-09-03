pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut q = VecDeque::new();
        for i in 1..10 {
            q.push_back(i);
        }
        for _ in 1..n {
            let m = q.len();
            for _ in 0..m {
                let num = q.pop_front().unwrap();
                let d = num % 10;
                let mut next = [d - k, d + k];
                if next[0] == next[1] {
                    next[1] = -1;
                }
                for d1 in next {
                    if (0..10).contains(&d1) {
                        q.push_back(num * 10 + d1);
                    }
                }
            }
        }
        q.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::nums_same_consec_diff(3, 7);
        result.sort_unstable();
        let mut expected = vec![181, 292, 707, 818, 929];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::nums_same_consec_diff(2, 1);
        result.sort_unstable();
        let mut expected = vec![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
        ];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
