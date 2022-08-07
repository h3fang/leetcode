pub struct Solution;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut r = vec![false; n as usize];
        for e in restricted {
            r[e as usize] = true;
        }
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut vis = vec![false; n as usize];
        vis[0] = true;
        let mut q = vec![0];
        let mut result = 1;
        while let Some(node) = q.pop() {
            for &next in &g[node as usize] {
                if !vis[next as usize] && !r[next as usize] {
                    vis[next as usize] = true;
                    result += 1;
                    q.push(next);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 7;
        let edges = [[0, 1], [1, 2], [3, 1], [4, 0], [0, 5], [5, 6]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        let restricted = vec![4, 5];
        assert_eq!(4, Solution::reachable_nodes(n, edges, restricted));
    }
}
