pub struct Solution;

use std::collections::HashMap;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::with_capacity(points.len() * 2);

        for p in points {
            *m.entry(p[1]).or_insert(0) += 1;
        }

        let (mut sum, mut ans) = (0i64, 0i64);
        for &v in m.values() {
            let c = (v * (v - 1) / 2) % MOD;
            ans = (ans + sum * c) % MOD;
            sum = (sum + c) % MOD;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 0], [2, 0], [3, 0], [2, 2], [3, 2]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(3, Solution::count_trapezoids(points));
    }

    #[test]
    fn case2() {
        let points = [[0, 0], [1, 0], [0, 1], [2, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(1, Solution::count_trapezoids(points));
    }
}
