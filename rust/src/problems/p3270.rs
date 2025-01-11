pub struct Solution;

impl Solution {
    pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
        let mut result = 0;
        for i in 0..4 {
            let d = (num1 % 10).min(num2 % 10).min(num3 % 10);
            result += 10i32.pow(i) * d;
            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::generate_key(1, 10, 1000));
    }

    #[test]
    fn case2() {
        assert_eq!(777, Solution::generate_key(987, 879, 798));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::generate_key(1, 2, 3));
    }
}
