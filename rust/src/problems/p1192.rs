use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![vec![]; n as usize];
        for c in &connections {
            g[c[0] as usize].push(c[1]);
            g[c[1] as usize].push(c[0]);
        }
        let mut connections: HashSet<[i32; 2]> = connections
            .into_iter()
            .map(|c| {
                if c[0] < c[1] {
                    [c[0], c[1]]
                } else {
                    [c[1], c[0]]
                }
            })
            .collect();
        let mut rank = vec![-2; n as usize];
        fn dfs(
            g: &[Vec<i32>],
            rank: &mut [i32],
            connections: &mut HashSet<[i32; 2]>,
            node: i32,
            depth: i32,
        ) -> i32 {
            if rank[node as usize] >= 0 {
                return rank[node as usize];
            }
            rank[node as usize] = depth;
            let mut min = i32::MAX;
            for &neighbor in &g[node as usize] {
                if rank[neighbor as usize] + 1 == depth {
                    continue;
                }
                let r = dfs(g, rank, connections, neighbor, depth + 1);
                if r <= depth {
                    let con = if node < neighbor {
                        [node, neighbor]
                    } else {
                        [neighbor, node]
                    };
                    connections.remove(&con);
                }
                min = min.min(r);
            }
            min
        }
        dfs(&g, &mut rank, &mut connections, 0, 0);
        connections.iter().map(|c| vec![c[0], c[1]]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_connections(cons: &[[i32; 2]]) -> Vec<Vec<i32>> {
        cons.iter().map(|c| c.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let n = 4;
        let connections = parse_connections(&[[0, 1], [1, 2], [2, 0], [1, 3]]);
        let mut expected = parse_connections(&[[1, 3]]);
        expected.sort_unstable();
        let mut result = Solution::critical_connections(n, connections);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
