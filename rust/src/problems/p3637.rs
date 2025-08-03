pub struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        for i in 1..n - 2 {
            for j in i + 1..n - 1 {
                if nums[..=i].windows(2).all(|w| w[0] < w[1])
                    && nums[i..=j].windows(2).all(|w| w[0] > w[1])
                    && nums[j..].windows(2).all(|w| w[0] < w[1])
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_trionic(vec![2, 1, 3]));
    }
}
