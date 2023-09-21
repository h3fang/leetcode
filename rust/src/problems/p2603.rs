pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let mut g = vec![vec![]; n];
        let mut degree = vec![0; n];
        for e in edges {
            let (x, y) = (e[0] as usize, e[1] as usize);
            g[x].push(y);
            g[y].push(x);
            degree[x] += 1;
            degree[y] += 1;
        }

        let mut m = n;

        let mut q = VecDeque::new();
        for (i, &d) in degree.iter().enumerate() {
            if d == 1 && coins[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(i) = q.pop_front() {
            degree[i] -= 1;
            m -= 1;
            for &j in &g[i] {
                degree[j] -= 1;
                if degree[j] == 1 && coins[j] == 0 {
                    q.push_back(j);
                }
            }
        }

        for _ in 0..2 {
            let mut q = VecDeque::new();
            for (i, &d) in degree.iter().enumerate() {
                if d == 1 {
                    q.push_back(i);
                }
            }
            while let Some(i) = q.pop_front() {
                degree[i] -= 1;
                m -= 1;
                for &j in &g[i] {
                    degree[j] -= 1;
                }
            }
        }

        2 * (m as i32 - 1).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let coins = vec![1, 0, 0, 0, 0, 1];
        let edges = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(2, Solution::collect_the_coins(coins, edges));
    }

    #[test]
    fn case2() {
        let coins = vec![0, 0, 0, 1, 1, 0, 0, 1];
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(2, Solution::collect_the_coins(coins, edges));
    }
}
