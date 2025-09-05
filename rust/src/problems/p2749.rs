pub struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let (mut num1, num2) = (num1 as i64, num2 as i64);
        for i in 1..=60 {
            num1 -= num2;
            if i > num1 {
                return -1;
            }
            if i as u32 >= num1.count_ones() {
                return i as i32;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::make_the_integer_zero(3, -2));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::make_the_integer_zero(5, 7));
    }
}
