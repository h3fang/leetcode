pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut a, mut b) = (0, 0);
        for bill in bills {
            match bill {
                5 => a += 1,
                10 => {
                    a -= 1;
                    b += 1;
                }
                _ => {
                    if b > 0 {
                        b -= 1;
                        a -= 1
                    } else {
                        a -= 3;
                    }
                }
            }
            if a < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }
}
