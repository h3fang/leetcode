pub struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut result = 0;
        for a in (n - 2 * limit).max(0)..=limit.min(n) {
            let n = n - a;
            result += limit.min(n) - (n - limit).max(0) + 1;
        }
        result
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
