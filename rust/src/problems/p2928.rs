pub struct Solution;

fn c2(n: i32) -> i32 {
    if n > 1 { n * (n - 1) / 2 } else { 0 }
}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        c2(n + 2) - 3 * c2(n - (limit + 1) + 2) + 3 * c2(n - 2 * (limit + 1) + 2)
            - c2(n - 3 * (limit + 1) + 2)
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

    #[test]
    fn case3() {
        assert_eq!(0, Solution::distribute_candies(6, 1));
    }
}
