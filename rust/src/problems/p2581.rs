pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let s = guesses
            .into_iter()
            .map(|g| (g[0], g[1]))
            .collect::<HashSet<_>>();
        let (mut result, mut count) = (0, 0);
        fn dfs(x: i32, p: i32, g: &[Vec<i32>], s: &HashSet<(i32, i32)>, count: &mut i32) {
            for &y in &g[x as usize] {
                if y == p {
                    continue;
                }
                *count += i32::from(s.contains(&(x, y)));
                dfs(y, x, g, s, count);
            }
        }
        dfs(0, -1, &g, &s, &mut count);

        fn dp(
            x: i32,
            p: i32,
            count: i32,
            k: i32,
            g: &[Vec<i32>],
            s: &HashSet<(i32, i32)>,
            result: &mut i32,
        ) {
            if count >= k {
                *result += 1;
            }
            for &y in &g[x as usize] {
                if y == p {
                    continue;
                }
                dp(
                    y,
                    x,
                    count - i32::from(s.contains(&(x, y))) + i32::from(s.contains(&(y, x))),
                    k,
                    g,
                    s,
                    result,
                );
            }
        }
        dp(0, -1, count, k, &g, &s, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [1, 3], [4, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let guesses = [[1, 3], [0, 1], [1, 0], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let k = 3;
        assert_eq!(3, Solution::root_count(edges, guesses, k));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let guesses = [[1, 0], [3, 4], [2, 1], [3, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let k = 1;
        assert_eq!(5, Solution::root_count(edges, guesses, k));
    }
}
