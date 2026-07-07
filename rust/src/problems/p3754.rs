pub struct Solution;

impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let (mut sum, mut x, mut p) = (0, 0, 1);
        while n > 0 {
            let d = n % 10;
            if d != 0 {
                sum += d as i64;
                x += d as i64 * p;
                p *= 10;
            }
            n /= 10;
        }
        sum * x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12340, Solution::sum_and_multiply(10203004));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::sum_and_multiply(1000));
    }
}
