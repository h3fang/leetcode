pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i < right {
            if nums[i] % 2 == 0 {
                nums.swap(left, i);
                left += 1;
                i += 1;
            } else {
                nums.swap(right, i);
                right -= 1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
        println!("{result:?}");
        if let Some(first_odd) = result.iter().position(|&n| n % 2 == 1) {
            assert!(result[first_odd + 1..].iter().all(|&n| n % 2 == 1));
        }
    }
}
