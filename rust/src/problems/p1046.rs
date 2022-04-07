use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut q = stones.into_iter().collect::<BinaryHeap<_>>();
        while q.len() >= 2 {
            let a = q.pop().unwrap();
            let b = q.pop().unwrap();
            if a > b {
                q.push(a - b);
            }
        }
        q.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::last_stone_weight(vec![1]));
    }
}
