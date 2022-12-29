pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(i, t)| (t[0], t[1], i as i32))
            .collect::<Vec<_>>();
        tasks.sort_unstable();
        let mut result = Vec::with_capacity(tasks.len());
        let mut q = BinaryHeap::new();
        q.push((Reverse(tasks[0].1), Reverse(tasks[0].2), tasks[0].0));
        let mut j = 1;
        let mut curr = 0;
        while let Some((Reverse(process), Reverse(i), enqueue)) = q.pop() {
            result.push(i);
            curr = curr.max(enqueue) + process;
            while j < tasks.len() && tasks[j].0 <= curr {
                q.push((Reverse(tasks[j].1), Reverse(tasks[j].2), tasks[j].0));
                j += 1;
            }
            if q.is_empty() && j < tasks.len() {
                q.push((Reverse(tasks[j].1), Reverse(tasks[j].2), tasks[j].0));
                j += 1;
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
        let tasks = [[1, 2], [2, 4], [3, 2], [4, 1]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(vec![0, 2, 3, 1], Solution::get_order(tasks));
    }

    #[test]
    fn case2() {
        let tasks = [[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(vec![4, 3, 2, 0, 1], Solution::get_order(tasks));
    }
}
