pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        fn dfs(
            nums: &[i32],
            g: &[Vec<usize>],
            i: usize,
            visited: &mut [bool],
            genes: &mut HashSet<i32>,
        ) {
            if visited[i] {
                return;
            }
            visited[i] = true;
            genes.insert(nums[i]);
            for &c in &g[i] {
                dfs(nums, g, c, visited, genes);
            }
        }
        let n = parents.len();
        let mut i = if let Some(i) = nums.iter().position(|&x| x == 1) {
            i
        } else {
            return vec![1; n];
        };
        let mut g = vec![vec![]; n];
        for (i, &p) in parents.iter().enumerate().skip(1) {
            g[p as usize].push(i);
        }
        let mut visited = vec![false; n];
        let mut genes = HashSet::new();
        let mut min = 1;
        let mut result = vec![1; n];
        loop {
            dfs(&nums, &g, i, &mut visited, &mut genes);
            while genes.contains(&min) {
                min += 1;
            }
            result[i] = min;
            let p = parents[i];
            if p == -1 {
                break;
            } else {
                i = p as usize;
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
        assert_eq!(
            vec![5, 1, 1, 1],
            Solution::smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![7, 1, 1, 4, 2, 1],
            Solution::smallest_missing_value_subtree(
                vec![-1, 0, 1, 0, 3, 3],
                vec![5, 4, 6, 2, 1, 3]
            )
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1],
            Solution::smallest_missing_value_subtree(
                vec![-1, 2, 3, 0, 2, 4, 1],
                vec![2, 3, 4, 5, 6, 7, 8]
            )
        );
    }
}
