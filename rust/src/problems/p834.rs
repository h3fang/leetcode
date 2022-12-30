pub struct Solution;

fn dfs(tree: &[Vec<i32>], size: &mut Vec<i32>, dp: &mut Vec<i32>, root: i32, pre: i32) {
    for &i in &tree[root as usize] {
        if i == pre {
            continue;
        }
        dfs(tree, size, dp, i, root);
        size[root as usize] += size[i as usize];
        dp[root as usize] += dp[i as usize] + size[i as usize];
    }
    size[root as usize] += 1;
}

fn change_root(tree: &[Vec<i32>], size: &mut Vec<i32>, dp: &mut Vec<i32>, root: i32, pre: i32) {
    for &i in &tree[root as usize] {
        if i == pre {
            continue;
        }
        dp[i as usize] =
            dp[root as usize] - size[i as usize] + size.len() as i32 - size[i as usize];
        change_root(tree, size, dp, i, root);
    }
}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut dp = vec![0; n];
        let mut size = vec![0; n];
        for e in &edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        dfs(&g, &mut size, &mut dp, 0, -1);
        change_root(&g, &mut size, &mut dp, 0, -1);
        dp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![8, 12, 6, 10, 10, 10],
            Solution::sum_of_distances_in_tree(6, edges)
        );
    }

    #[test]
    fn case2() {
        let edges = vec![];
        assert_eq!(vec![0], Solution::sum_of_distances_in_tree(1, edges));
    }

    #[test]
    fn case3() {
        let edges = [[1, 0]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![1, 1], Solution::sum_of_distances_in_tree(2, edges));
    }
}
