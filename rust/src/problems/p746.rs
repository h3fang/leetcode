pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut f = vec![i32::MAX; cost.len() + 2];
        f[0] = 0;
        f[1] = 0;
        for (i, &c) in cost.iter().enumerate() {
            f[i + 1] = f[i + 1].min(f[i] + c);
            f[i + 2] = f[i + 2].min(f[i] + c);
        }
        f[cost.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
