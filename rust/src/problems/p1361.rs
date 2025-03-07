pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        fn dfs(seen: &mut [bool], left: &[i32], right: &[i32], i: usize) -> bool {
            if seen[i] {
                return false;
            }
            seen[i] = true;
            (left[i] == -1 || dfs(seen, left, right, left[i] as usize))
                && (right[i] == -1 || dfs(seen, left, right, right[i] as usize))
        }

        let children = left_child
            .iter()
            .chain(&right_child)
            .cloned()
            .collect::<HashSet<_>>();
        let root = if let Some(r) = (0..n).find(|i| !(children.contains(i))) {
            r
        } else {
            return false;
        };
        let mut seen = vec![false; n as usize];
        dfs(&mut seen, &left_child, &right_child, root as usize) && seen.iter().all(|&e| e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::validate_binary_tree_nodes(
            4,
            vec![1, -1, 3, -1],
            vec![2, -1, -1, -1]
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::validate_binary_tree_nodes(
            4,
            vec![1, -1, 3, -1],
            vec![2, 3, -1, -1]
        ));
    }

    #[test]
    fn case3() {
        assert!(!Solution::validate_binary_tree_nodes(
            2,
            vec![1, 0],
            vec![-1, -1]
        ));
    }
}
