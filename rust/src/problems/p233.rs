pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut result = 0;
        let mut k = 1;
        while n >= k {
            result += (n / (k * 10)) * k + (n % (k * 10) - k + 1).max(0).min(k);
            k *= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_digit_one(13));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_digit_one(0));
    }

    #[test]
    fn case3() {
        assert_eq!(102, Solution::count_digit_one(164));
    }
}
