pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut index = (0..queries.len()).collect::<Vec<_>>();
        index.sort_unstable_by_key(|&i| queries[i]);
        intervals.sort_unstable_by_key(|i| i[0]);
        let mut q = BinaryHeap::new();
        let mut result = vec![-1; queries.len()];
        let mut i = 0;
        for j in index {
            while i < intervals.len() && intervals[i][0] <= queries[j] {
                let c = intervals[i][1] - intervals[i][0] + 1;
                q.push((Reverse(c), intervals[i][0], intervals[i][1]));
                i += 1;
            }
            while !q.is_empty() && q.peek().unwrap().2 < queries[j] {
                q.pop();
            }
            if let Some(&(Reverse(c), _, _)) = q.peek() {
                result[j] = c;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let intervals = [[1, 4], [2, 4], [3, 6], [4, 4]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        let queries = vec![2, 3, 4, 5];
        assert_eq!(vec![3, 3, 1, 4], Solution::min_interval(intervals, queries));
    }

    #[test]
    fn case2() {
        let intervals = [[2, 3], [2, 5], [1, 8], [20, 25]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        let queries = vec![2, 19, 5, 22];
        assert_eq!(
            vec![2, -1, 4, 6],
            Solution::min_interval(intervals, queries)
        );
    }
}
