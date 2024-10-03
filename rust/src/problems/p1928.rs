pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let n = passing_fees.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], e[2]));
        }
        let mut fees = vec![vec![i32::MAX; max_time as usize + 1]; n];
        let mut q: BinaryHeap<(Reverse<i32>, i32, usize)> = BinaryHeap::new();
        q.push((Reverse(passing_fees[0]), 0, 0));
        while let Some((Reverse(f), t, i)) = q.pop() {
            if f > fees[i][t as usize] {
                continue;
            }
            for &(j, dt) in &g[i] {
                let t1 = t + dt;
                if t1 > max_time {
                    continue;
                }
                let j = j as usize;
                let f1 = f + passing_fees[j];
                if f1 < fees[j][t1 as usize] {
                    fees[j][t1 as usize] = f1;
                    q.push((Reverse(f1), t1, j));
                }
            }
        }
        let min = *fees[n - 1].iter().min().unwrap();
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [
            [0, 1, 10],
            [1, 2, 10],
            [2, 5, 10],
            [0, 3, 1],
            [3, 4, 10],
            [4, 5, 15],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let passing_fees = vec![5, 1, 2, 20, 20, 3];
        assert_eq!(11, Solution::min_cost(30, edges, passing_fees));
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 1, 10],
            [1, 2, 10],
            [2, 5, 10],
            [0, 3, 1],
            [3, 4, 10],
            [4, 5, 15],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let passing_fees = vec![5, 1, 2, 20, 20, 3];
        assert_eq!(48, Solution::min_cost(29, edges, passing_fees));
    }

    #[test]
    fn case3() {
        let edges = [
            [0, 1, 10],
            [1, 2, 10],
            [2, 5, 10],
            [0, 3, 1],
            [3, 4, 10],
            [4, 5, 15],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let passing_fees = vec![5, 1, 2, 20, 20, 3];
        assert_eq!(-1, Solution::min_cost(25, edges, passing_fees));
    }
}
