pub struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if (connections.len() as i32) < n - 1 {
            return -1;
        }

        fn dfs(g: &[Vec<i32>], i: usize, visited: &mut [bool]) {
            visited[i] = true;
            for &n in &g[i] {
                if !visited[n as usize] {
                    dfs(g, n as usize, visited);
                }
            }
        }

        let mut g = vec![vec![]; n as usize];
        for c in connections {
            g[c[0] as usize].push(c[1]);
            g[c[1] as usize].push(c[0]);
        }
        let mut comps = 0;
        let mut visited = vec![false; n as usize];
        for i in 0..n as usize {
            if !visited[i] {
                dfs(&g, i, &mut visited);
                comps += 1;
            }
        }
        comps - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let connections = [[0, 1], [0, 2], [1, 2]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(1, Solution::make_connected(4, connections));
    }

    #[test]
    fn case2() {
        let connections = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(2, Solution::make_connected(6, connections));
    }

    #[test]
    fn case3() {
        let connections = [[0, 1], [0, 2], [0, 3], [1, 2]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(-1, Solution::make_connected(6, connections));
    }
}
