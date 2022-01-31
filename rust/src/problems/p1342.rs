pub struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut result = 0;
        while num > 0 {
            if num & 1 == 1 {
                num -= 1;
            } else {
                num >>= 1;
            }
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::number_of_steps(14));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::number_of_steps(8));
    }
}
