pub struct Solution;

impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            nums[i] = 1000 * (nums[nums[i] as usize] % 1000) + nums[i];
        }
        nums.iter_mut().for_each(|x| *x /= 1000);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![0, 1, 2, 4, 5, 3],
            Solution::build_array(vec![0, 2, 1, 5, 3, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![4, 5, 0, 1, 2, 3],
            Solution::build_array(vec![5, 0, 1, 2, 3, 4])
        );
    }
}
