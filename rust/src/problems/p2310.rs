pub struct Solution;

impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        for n in 1..=10 {
            if num - n * k < 0 {
                break;
            }
            if (num - n * k) % 10 == 0 {
                return n;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_numbers(58, 9));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::minimum_numbers(37, 2));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_numbers(0, 7));
    }

    #[test]
    fn case4() {
        assert_eq!(-1, Solution::minimum_numbers(4, 0));
    }

    #[test]
    fn case5() {
        assert_eq!(1, Solution::minimum_numbers(10, 0));
    }
}
