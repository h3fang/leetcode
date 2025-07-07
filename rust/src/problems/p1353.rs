pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[0]);
        let n = events.len();
        let max = events.iter().map(|e| e[1]).max().unwrap();
        let mut ans = 0;
        let mut i = 0;
        let mut q = BinaryHeap::with_capacity(n);
        for d in 1..=max {
            while i < n && d >= events[i][0] {
                q.push(Reverse(events[i][1]));
                i += 1;
            }
            while q.peek().is_some_and(|e| e.0 < d) {
                q.pop();
            }
            if !q.is_empty() {
                q.pop();
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let events = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::max_events(events));
    }

    #[test]
    fn case2() {
        let events = [[1, 2], [2, 3], [3, 4], [1, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(4, Solution::max_events(events));
    }
}
