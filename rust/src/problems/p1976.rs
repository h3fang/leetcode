pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for r in roads {
            g[r[0] as usize].push((r[1] as usize, r[2] as i64));
            g[r[1] as usize].push((r[0] as usize, r[2] as i64));
        }
        let mut dis = vec![i64::MAX / 2; n];
        dis[0] = 0;
        let mut ways = vec![0; n];
        ways[0] = 1;
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0));
        while let Some((Reverse(d), i)) = q.pop() {
            if d > dis[i] {
                continue;
            }
            for &(j, w) in &g[i] {
                match (d + w).cmp(&dis[j]) {
                    std::cmp::Ordering::Less => {
                        dis[j] = d + w;
                        ways[j] = ways[i];
                        q.push((Reverse(d + w), j));
                    }
                    std::cmp::Ordering::Equal => {
                        ways[j] = (ways[i] + ways[j]) % 10_0000_0007;
                    }
                    std::cmp::Ordering::Greater => {}
                }
            }
        }
        ways[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let roads = [
            [0, 6, 7],
            [0, 1, 2],
            [1, 2, 3],
            [1, 3, 3],
            [6, 3, 3],
            [3, 5, 1],
            [6, 5, 1],
            [2, 5, 1],
            [0, 4, 5],
            [4, 6, 2],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(4, Solution::count_paths(7, roads));
    }

    #[test]
    fn case2() {
        let roads = [[1, 0, 10]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::count_paths(2, roads));
    }
}
