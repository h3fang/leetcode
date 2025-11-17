pub struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut zeros = k;
        for x in nums {
            if x == 1 {
                if zeros < k {
                    return false;
                }
                zeros = 0;
            } else {
                zeros += 1;
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
        assert!(Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2));
    }

    #[test]
    fn case2() {
        assert!(!Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
    }
}
