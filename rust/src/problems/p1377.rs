pub struct Solution;

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut g = vec![vec![]; n as usize + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        fn dfs(g: &[Vec<i32>], target: i32, t: i32, visited: &mut [bool], i: i32) -> f64 {
            let n = if i > 1 {
                g[i as usize].len() - 1
            } else {
                g[i as usize].len()
            };

            if t == 0 || n == 0 {
                return if i == target { 1.0 } else { 0.0 };
            }
            visited[i as usize] = true;
            let mut result = 0.0;
            for &c in &g[i as usize] {
                if visited[c as usize] {
                    continue;
                }
                result += dfs(g, target, t - 1, visited, c);
            }
            result / (n as f64)
        }
        let mut visited = vec![false; n as usize + 1];
        dfs(&g, target, t, &mut visited, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {:.5}, b = {:.5}", a, b);
    }

    #[test]
    fn case1() {
        let edges = [[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_close(0.166666, Solution::frog_position(7, edges, 2, 4));
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_close(0.333333, Solution::frog_position(7, edges, 1, 7));
    }
}
