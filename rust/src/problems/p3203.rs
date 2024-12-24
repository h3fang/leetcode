pub struct Solution;

fn dfs(g: &[Vec<i32>], p: i32, u: i32, d: i32, max: &mut (i32, i32)) {
    for &v in &g[u as usize] {
        if v == p {
            continue;
        }
        if d + 1 > max.0 {
            max.0 = d + 1;
            max.1 = v;
        }
        dfs(g, u, v, d + 1, max);
    }
}

fn diameter(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for e in edges {
        g[e[0] as usize].push(e[1]);
        g[e[1] as usize].push(e[0]);
    }
    let mut max = (0, 0);
    dfs(&g, -1, 0, 0, &mut max);
    let c = max.1;
    max = (0, 0);
    dfs(&g, -1, c, 0, &mut max);
    max.0
}

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = diameter(edges1);
        let d2 = diameter(edges2);
        d1.max(d2).max((d1 + 1) / 2 + (d2 + 1) / 2 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges1 = [[0, 1], [0, 2], [0, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let edges2 = [[0, 1]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(3, Solution::minimum_diameter_after_merge(edges1, edges2));
    }

    #[test]
    fn case2() {
        let edges1 = [[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let edges2 = [[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(5, Solution::minimum_diameter_after_merge(edges1, edges2));
    }
}
