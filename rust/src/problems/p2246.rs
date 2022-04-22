pub struct Solution;

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        fn dfs(g: &[Vec<usize>], s: &[u8], node: usize, result: &mut i32) -> i32 {
            let mut max_x = 0;
            for &child in &g[node] {
                let max_y = dfs(g, s, child, result) + 1;
                if s[node] != s[child] {
                    *result = (*result).max(max_x + max_y);
                    max_x = max_x.max(max_y);
                }
            }
            max_x
        }
        let n = parent.len();
        let mut g = vec![vec![]; n];
        for (i, &p) in parent.iter().enumerate().skip(1) {
            g[p as usize].push(i);
        }
        let mut result = 0;
        dfs(&g, s.as_bytes(), 0, &mut result);
        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::longest_path(vec![-1, 0, 0, 0], "aabc".into()));
    }
}
