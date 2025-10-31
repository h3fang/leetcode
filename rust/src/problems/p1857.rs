pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut g = vec![vec![]; n];
        let mut indegree = vec![0; n];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            indegree[e[1] as usize] += 1;
        }
        let mut dp = vec![vec![0; 26]; n];
        let mut q = VecDeque::new();
        for (i, &d) in indegree.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }
        let mut result = 0;
        let mut visited = 0;
        let colors = colors.as_bytes();
        while let Some(x) = q.pop_front() {
            dp[x][(colors[x] - b'a') as usize] += 1;
            result = result.max(dp[x][(colors[x] - b'a') as usize]);
            visited += 1;
            for &y in &g[x] {
                let y = y as usize;
                #[allow(clippy::needless_range_loop)]
                for i in 0..26 {
                    dp[y][i] = dp[y][i].max(dp[x][i]);
                }
                indegree[y] -= 1;
                if indegree[y] == 0 {
                    q.push_back(y);
                }
            }
        }
        if visited != n { -1 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let colors = "abaca".to_string();
        let edges = [[0, 1], [0, 2], [2, 3], [3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::largest_path_value(colors, edges));
    }

    #[test]
    fn case2() {
        let colors = "a".to_string();
        let edges = [[0, 0]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(-1, Solution::largest_path_value(colors, edges));
    }
}
