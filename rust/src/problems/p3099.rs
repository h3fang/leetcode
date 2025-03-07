pub struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let (mut y, mut sum) = (x, 0);
        while y > 0 {
            sum += y % 10;
            y /= 10;
        }
        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::sum_of_the_digits_of_harshad_number(18));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::sum_of_the_digits_of_harshad_number(23));
    }
}
