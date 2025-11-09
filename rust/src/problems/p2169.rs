pub struct Solution;

impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut ans = 0;
        while num2 > 0 {
            ans += num1 / num2;
            (num1, num2) = (num2, num1 % num2);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_operations(2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_operations(10, 10));
    }
}
