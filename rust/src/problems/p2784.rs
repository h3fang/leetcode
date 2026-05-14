pub struct Solution;

impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        let (n, mut c) = (nums.len() - 1, 0);
        for i in 0..=n {
            let x = nums[i].abs();
            if x > n as i32 || (x == n as i32 && c == 2) || ((x < n as i32) && nums[x as usize] < 0)
            {
                return false;
            }
            if x == n as i32 {
                c += 1;
            }
            nums[x as usize] *= -1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::is_good(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_good(vec![1, 3, 3, 2]));
    }

    #[test]
    fn case3() {
        assert!(Solution::is_good(vec![1, 1]));
    }

    #[test]
    fn case4() {
        assert!(!Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
    }
}
