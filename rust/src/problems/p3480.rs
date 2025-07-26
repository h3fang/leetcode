pub struct Solution;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let mut g = vec![[n + 1; 2]; n as usize + 1];
        for p in conflicting_pairs {
            let (a, b) = (p[0].min(p[1]), p[0].max(p[1]));
            let g = &mut g[a as usize];
            if b < g[0] {
                g[1] = g[0];
                g[0] = b;
            } else if b < g[1] {
                g[1] = b;
            }
        }
        let (mut ans, mut max, mut extra) = (0, 0, 0);
        let (mut b0, mut b1) = (n + 1, n + 1);
        for i in (1..=n).rev() {
            let pre = b0;
            for &b in &g[i as usize] {
                if b < b0 {
                    (b0, b1) = (b, b0);
                } else if b < b1 {
                    b1 = b;
                }
            }
            ans += (b0 - i) as i64;
            if b0 != pre {
                extra = 0;
            }
            extra += (b1 - b0) as i64;
            max = max.max(extra);
        }
        ans + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let conflicting_pairs = [[2, 3], [1, 4]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(9, Solution::max_subarrays(4, conflicting_pairs));
    }

    #[test]
    fn case2() {
        let conflicting_pairs = [[1, 2], [2, 5], [3, 5]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(12, Solution::max_subarrays(5, conflicting_pairs));
    }
}
