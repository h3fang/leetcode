pub struct Solution;

impl Solution {
    pub fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = i64::from(n);
        let mut ans = (2 * n * n + 5 * n - 6) * (n - 1) / 6;
        if n == edges.len() as i64 {
            ans += 2;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(23, Solution::max_score(4, edges));
    }

    #[test]
    fn case2() {
        let edges = [[0, 3], [4, 5], [2, 0], [1, 3], [2, 4], [1, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(82, Solution::max_score(6, edges));
    }
}
