pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut result = reward2.iter().sum::<i32>();
        let mut q: BinaryHeap<i32> = reward1.iter().zip(&reward2).map(|(&a, &b)| a - b).collect();
        for _ in 0..k {
            let v = q.pop().unwrap();
            result += v;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            15,
            Solution::mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::mice_and_cheese(vec![1, 1], vec![1, 1], 2));
    }
}
