pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_spending(mut values: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (values.len(), values[0].len());
        let mut q = BinaryHeap::new();
        for (i, v) in values.iter_mut().enumerate() {
            let v = v.pop().unwrap();
            q.push((Reverse(v), i));
        }
        let mut result = 0;
        for d in 1..=m * n {
            let (Reverse(v), i) = q.pop().unwrap();
            result += d as i64 * v as i64;
            if let Some(v) = values[i].pop() {
                q.push((Reverse(v), i));
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
        let values = [[8, 5, 2], [6, 4, 1], [9, 7, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(285, Solution::max_spending(values));
    }

    #[test]
    fn case2() {
        let values = [[10, 8, 6, 4, 2], [9, 7, 5, 3, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(386, Solution::max_spending(values));
    }
}
