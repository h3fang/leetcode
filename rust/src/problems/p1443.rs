pub struct Solution;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        fn dfs(g: &[Vec<i32>], has_apple: &[bool], node: i32, visited: &mut [bool]) -> (i32, bool) {
            let (mut result, mut apple) = (0, has_apple[node as usize]);
            visited[node as usize] = true;
            for &child in &g[node as usize] {
                if visited[child as usize] {
                    continue;
                }
                let (r, a) = dfs(g, has_apple, child, visited);
                if a {
                    result += r + 2;
                    apple = true;
                }
            }
            (result, apple)
        }
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut visited = vec![false; n as usize];
        dfs(&g, &has_apple, 0, &mut visited).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 7;
        let edges = [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let has_apple = vec![false, false, true, false, true, true, false];
        assert_eq!(8, Solution::min_time(n, edges, has_apple));
    }

    #[test]
    fn case2() {
        let n = 7;
        let edges = [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let has_apple = vec![false, false, true, false, false, true, false];
        assert_eq!(6, Solution::min_time(n, edges, has_apple));
    }
}
