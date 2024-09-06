pub struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut sum = 1;
        for i in 2..num {
            if i * i > num {
                break;
            }
            if num % i == 0 {
                sum += i;
                let r = num / i;
                if r != i {
                    sum += r;
                }
            }
        }
        sum != 1 && sum == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_perfect_number(28));
    }

    #[test]
    fn case2() {
        assert!(Solution::check_perfect_number(496));
    }

    #[test]
    fn case3() {
        assert!(!Solution::check_perfect_number(2));
    }
}
