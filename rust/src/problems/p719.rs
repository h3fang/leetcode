pub struct Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        fn count(nums: &[i32], m: i32) -> i32 {
            let mut c = 0;
            let mut i = 0;
            for (j, &b) in nums.iter().enumerate() {
                while b - nums[i] > m {
                    i += 1;
                }
                c += j - i;
            }
            c as i32
        }

        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];
        while left <= right {
            let mid = left + (right - left) / 2;
            match count(&nums, mid).cmp(&k) {
                std::cmp::Ordering::Less => left = mid + 1,
                _ => right = mid - 1,
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::smallest_distance_pair(vec![1, 3, 1], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::smallest_distance_pair(vec![1, 6, 1], 3));
    }
}
