pub struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = Vec::with_capacity(n);
        let mut m = 0;
        for &x in &nums {
            if x > 0 {
                result.push(x);
                while nums[m] > 0 {
                    m += 1;
                }
                result.push(nums[m]);
                m += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, -2, 1, -5, 2, -4],
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, -1], Solution::rearrange_array(vec![-1, 1]));
    }
}
