pub struct Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        result[n - 1] = 1;
        let mut count = [0; 32];
        for (j, c) in count.iter_mut().enumerate() {
            if nums[n - 1] & (1 << j) > 0 {
                *c += 1;
            }
        }
        for i in (0..n - 1).rev() {
            let a = nums[i];
            for (j, c) in count.iter_mut().enumerate() {
                if a & (1 << j) > 0 {
                    *c += 1;
                }
            }
            result[i] = result[i + 1] + 1;
            for k in (i + 1..i + 1 + result[i + 1] as usize).rev() {
                if (0..32).all(|j| nums[k] & (1 << j) == 0 || count[j] > 1) {
                    for (j, c) in count.iter_mut().enumerate() {
                        if nums[k] & (1 << j) > 0 {
                            *c -= 1;
                        }
                    }
                    result[i] -= 1;
                } else {
                    break;
                }
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
            vec![3, 3, 2, 2, 1],
            Solution::smallest_subarrays(vec![1, 0, 2, 1, 3])
        );
    }
}
