pub struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }

        let half = 10i32.pow(n as u32) - 1;
        for left in ((half / 10)..=half).rev() {
            let mut p = left as i64;
            let mut right = left as i64;
            while right > 0 {
                p = p * 10 + right % 10;
                right /= 10;
            }
            let mut x = half as i64;
            while x * x >= p {
                if p % x == 0 {
                    return (p % 1337) as i32;
                }
                x -= 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(987, Solution::largest_palindrome(2));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::largest_palindrome(1));
    }
}
