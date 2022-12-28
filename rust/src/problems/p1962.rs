pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut q = piles.into_iter().collect::<BinaryHeap<_>>();
        for _ in 0..k {
            let n = q.pop().unwrap();
            let r = n - n / 2;
            if r > 0 {
                q.push(r);
            }
        }
        q.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::min_stone_sum(vec![5, 4, 9], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::min_stone_sum(vec![4, 3, 6, 7], 3));
    }
}
