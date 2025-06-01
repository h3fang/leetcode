pub struct Solution;

fn f(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    n * (n - 1) / 2
}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let (n, limit) = (n as i64, limit as i64);
        f(n + 2) - 3 * f(n - (limit + 1) + 2) + 3 * f(n - 2 * (limit + 1) + 2)
            - f(n - 3 * (limit + 1) + 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::distribute_candies(5, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::distribute_candies(3, 3));
    }
}
