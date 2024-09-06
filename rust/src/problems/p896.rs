pub struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut tone = None;
        for (i, n) in nums.iter().enumerate() {
            if i > 0 {
                match tone {
                    None => {
                        if *n != nums[i - 1] {
                            tone = Some(*n > nums[i - 1]);
                        }
                    }
                    Some(ord) => {
                        if ord && *n < nums[i - 1] {
                            return false;
                        }

                        if !ord && *n > nums[i - 1] {
                            return false;
                        }
                    }
                }
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
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }

    #[test]
    fn case4() {
        assert!(Solution::is_monotonic(vec![1]));
    }

    #[test]
    fn case5() {
        assert!(Solution::is_monotonic(vec![1, 1, 1]));
    }
}
