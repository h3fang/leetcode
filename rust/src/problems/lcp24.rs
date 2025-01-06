pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn nums_game(nums: Vec<i32>) -> Vec<i32> {
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        let (mut left_sum, mut right_sum) = (0, 0);
        nums.into_iter()
            .enumerate()
            .map(|(i, x)| {
                let x = x as i64 - i as i64;
                if i % 2 == 0 {
                    left_sum += x;
                    left.push(x);
                    let x = left.pop().unwrap();
                    left_sum -= x;
                    right.push(Reverse(x));
                    right_sum += x;
                    ((right_sum - right.peek().unwrap().0 - left_sum) % MOD) as i32
                } else {
                    right_sum += x;
                    right.push(Reverse(x));
                    let x = right.pop().unwrap().0;
                    right_sum -= x;
                    left.push(x);
                    left_sum += x;
                    ((right_sum - left_sum) % MOD) as i32
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![0, 0, 0, 5, 6, 7],
            Solution::nums_game(vec![3, 4, 5, 1, 6, 7])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 0, 0, 0, 0],
            Solution::nums_game(vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![0, 1, 2, 3, 3, 3],
            Solution::nums_game(vec![1, 1, 1, 2, 3, 4])
        );
    }
}
