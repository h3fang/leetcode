pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());
        let mut cost = vec![i32::MAX; n * m];
        let mut q = BinaryHeap::with_capacity(n * m);
        q.push((Reverse(0i32), 0i32, 0i32));
        while let Some((Reverse(t), i, j)) = q.pop() {
            if i == n as i32 - 1 && j == m as i32 - 1 {
                return t;
            }
            if t > cost[i as usize * m + j as usize] {
                continue;
            }
            let dt = (i + j) % 2 + 1;
            for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if i1 < 0 || j1 < 0 || i1 == n as i32 || j1 == m as i32 {
                    continue;
                }
                let t1 = move_time[i1 as usize][j1 as usize].max(t) + dt;
                if t1 < cost[i1 as usize * m + j1 as usize] {
                    cost[i1 as usize * m + j1 as usize] = t1;
                    q.push((Reverse(t1), i1, j1));
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let move_time = [[0, 4], [4, 4]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(7, Solution::min_time_to_reach(move_time));
    }

    #[test]
    fn case2() {
        let move_time = [[0, 0, 0, 0], [0, 0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(6, Solution::min_time_to_reach(move_time));
    }

    #[test]
    fn case3() {
        let move_time = [[0, 1], [1, 2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::min_time_to_reach(move_time));
    }
}
