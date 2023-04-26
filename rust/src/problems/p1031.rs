pub struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        fn f(nums: &[i32], first_len: usize, second_len: usize) -> i32 {
            let mut suml = nums[..first_len].iter().sum::<i32>();
            let mut max = suml;
            let mut sumr = nums[first_len..(first_len + second_len)]
                .iter()
                .sum::<i32>();
            let mut result = max + sumr;
            for i in (first_len + second_len)..nums.len() {
                let j = i - second_len;
                suml += nums[j] - nums[j - first_len];
                max = max.max(suml);
                sumr += nums[i] - nums[i - second_len];
                result = result.max(sumr + max);
            }
            result
        }
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        f(&nums, first_len, second_len).max(f(&nums, second_len, first_len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            20,
            Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            29,
            Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            31,
            Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3)
        );
    }
}
