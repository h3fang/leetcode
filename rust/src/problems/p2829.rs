pub struct Solution;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        if n <= k / 2 {
            (1 + n) * n / 2
        } else {
            let r = n - k / 2;
            (1 + k / 2) * (k / 2) / 2 + (k + k + r - 1) * r / 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(18, Solution::minimum_sum(5, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_sum(2, 6));
    }

    #[test]
    fn case3() {
        assert_eq!(13, Solution::minimum_sum(4, 7));
    }
}
