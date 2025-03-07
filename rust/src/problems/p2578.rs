pub struct Solution;

impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(10);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort_unstable();
        let a = digits.iter().step_by(2).fold(0, |a, b| a * 10 + b);
        let b = digits.iter().skip(1).step_by(2).fold(0, |a, b| a * 10 + b);
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(59, Solution::split_num(4325));
    }

    #[test]
    fn case2() {
        assert_eq!(75, Solution::split_num(687));
    }
}
