pub struct Solution;

fn dfs(g: &[Vec<i32>], x: i32, vis: &mut [bool]) {
    vis[x as usize] = true;
    for &y in &g[x as usize] {
        if !vis[y as usize] {
            dfs(g, y, vis);
        }
    }
}

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[1] as usize].push(e[0]);
        }

        let mut ans = Vec::with_capacity(n);
        for x in 0..n {
            let mut vis = vec![false; n];
            dfs(&g, x as i32, &mut vis);
            vis[x] = false;
            let ancestors = vis
                .iter()
                .enumerate()
                .filter(|e| *e.1)
                .map(|e| e.0 as i32)
                .collect();
            ans.push(ancestors);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [
            [0, 3],
            [0, 4],
            [1, 3],
            [2, 4],
            [2, 7],
            [3, 5],
            [3, 6],
            [3, 7],
            [4, 6],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let expected = vec![
            vec![],
            vec![],
            vec![],
            vec![0, 1],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];
        assert_eq!(expected, Solution::get_ancestors(8, edges));
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [1, 2],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 4],
            [3, 4],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let expected = vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]];
        assert_eq!(expected, Solution::get_ancestors(5, edges));
    }
}
