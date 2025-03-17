pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut f = vec![false; 501];
        let mut odd = 0;
        for x in nums {
            f[x as usize] ^= true;
            if f[x as usize] {
                odd += 1;
            } else {
                odd -= 1;
            }
        }
        odd == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::divide_array(vec![1, 2, 3, 4]));
    }
}
