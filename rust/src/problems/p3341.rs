pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());
        let mut dist = vec![i32::MAX; n * m];
        let mut q = BinaryHeap::with_capacity(n * m);
        q.push((Reverse(0i32), 0i32, 0i32));
        while let Some((Reverse(d), i, j)) = q.pop() {
            if i == n as i32 - 1 && j == m as i32 - 1 {
                return d;
            }
            if d > dist[i as usize * m + j as usize] {
                continue;
            }
            for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if i1 < 0 || j1 < 0 || i1 == n as i32 || j1 == m as i32 {
                    continue;
                }
                let d1 = move_time[i1 as usize][j1 as usize].max(d) + 1;
                if d1 < dist[i1 as usize * m + j1 as usize] {
                    dist[i1 as usize * m + j1 as usize] = d1;
                    q.push((Reverse(d1), i1, j1));
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
        assert_eq!(6, Solution::min_time_to_reach(move_time));
    }

    #[test]
    fn case2() {
        let move_time = [[0, 0, 0], [0, 0, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::min_time_to_reach(move_time));
    }

    #[test]
    fn case3() {
        let move_time = [[0, 1], [1, 2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::min_time_to_reach(move_time));
    }
}
