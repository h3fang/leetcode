pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        let mut q: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for i in intervals {
            if !q.is_empty() && q.peek().unwrap().0 < i[0] {
                q.pop();
            }
            q.push(Reverse(i[1]));
        }
        q.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let intervals = [[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(3, Solution::min_groups(intervals));
    }

    #[test]
    fn case2() {
        let intervals = [[1, 3], [5, 6], [8, 10], [11, 13]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(1, Solution::min_groups(intervals));
    }
}
