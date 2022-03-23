pub struct Solution;

impl Solution {
    pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
        let mut result = 0;
        while target > start_value {
            if target % 2 == 0 {
                target /= 2;
                result += 1;
            } else {
                target = (target + 1) / 2;
                result += 2;
            }
        }
        result + start_value - target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::broken_calc(2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::broken_calc(5, 8));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::broken_calc(3, 10));
    }
}
