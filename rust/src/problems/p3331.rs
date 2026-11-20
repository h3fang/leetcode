pub struct Solution;

fn dfs(g: &[Vec<i32>], s: &[u8], x: i32, ans: &mut [i32], ancestors: &mut [i32]) {
    let sx = s[x as usize] - b'a';
    let old = ancestors[sx as usize];
    ancestors[sx as usize] = x;

    for &y in &g[x as usize] {
        dfs(g, s, y, ans, ancestors);
        let p = ancestors[(s[y as usize] - b'a') as usize];
        let p = if p == -1 { x } else { p };
        ans[p as usize] += ans[y as usize];
    }

    ancestors[sx as usize] = old;
}

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let mut g = vec![vec![]; n];
        for (i, &x) in parent.iter().enumerate().skip(1) {
            g[x as usize].push(i as i32);
        }

        let mut ans = vec![1; n];
        let mut ancestors = [-1; 26];

        dfs(&g, s.as_bytes(), 0, &mut ans, &mut ancestors);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let parent = vec![-1, 0, 0, 1, 1, 1];
        let s = "abaabc".to_string();
        assert_eq!(
            vec![6, 3, 1, 1, 1, 1],
            Solution::find_subtree_sizes(parent, s)
        );
    }

    #[test]
    fn case2() {
        let parent = vec![-1, 0, 4, 0, 1];
        let s = "abbba".to_string();
        assert_eq!(vec![5, 2, 1, 1, 1], Solution::find_subtree_sizes(parent, s));
    }
}
