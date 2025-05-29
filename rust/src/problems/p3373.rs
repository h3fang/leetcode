pub struct Solution;

fn num_of_colors(edges: Vec<Vec<i32>>, colors: &mut Option<Vec<bool>>) -> (i32, i32) {
    let n = edges.len() + 1;
    let mut g2 = vec![vec![]; n];
    for e in edges {
        g2[e[0] as usize].push(e[1]);
        g2[e[1] as usize].push(e[0]);
    }
    let r = dfs(&g2, 0, -1, 0, colors);
    (r, n as i32 - r)
}

fn dfs(g: &[Vec<i32>], x: i32, p: i32, d: i32, colors: &mut Option<Vec<bool>>) -> i32 {
    if let Some(c) = colors {
        c[x as usize] = d % 2 == 0;
    }
    let mut ans = 1 - d % 2;
    for &y in &g[x as usize] {
        if y == p {
            continue;
        }
        ans += dfs(g, y, x, d + 1, colors);
    }
    ans
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let mut colors = Some(vec![false; n]);
        let (w1, b1) = num_of_colors(edges1, &mut colors);
        let (w2, b2) = num_of_colors(edges2, &mut None);
        let max = w2.max(b2);
        colors
            .unwrap()
            .into_iter()
            .map(|c| if c { w1 } else { b1 } + max)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges1 = [[0, 1], [0, 2], [2, 3], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let edges2 = [[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![8, 7, 7, 8, 8],
            Solution::max_target_nodes(edges1, edges2)
        );
    }

    #[test]
    fn case2() {
        let edges1 = [[0, 1], [0, 2], [0, 3], [0, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let edges2 = [[0, 1], [1, 2], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![3, 6, 6, 6, 6],
            Solution::max_target_nodes(edges1, edges2)
        );
    }
}
