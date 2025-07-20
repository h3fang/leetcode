pub struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let (mut sum, mut prod) = (0, 1);
        let mut x = n;
        while x > 0 {
            let d = x % 10;
            sum += d;
            prod *= d;
            x /= 10;
        }
        n % (sum + prod) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_divisibility(99));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_divisibility(23));
    }
}
