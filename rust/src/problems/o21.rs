pub struct Solution;

impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i < right {
            while i < right && nums[i] % 2 == 0 {
                nums.swap(i, right);
                right -= 1;
            }
            i += 1;
        }
        nums
    }
}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 4];
        let results = Solution::exchange(nums);
        if let Some(p) = results.iter().position(|&x| x % 2 == 0) {
            assert!(results[p..].iter().all(|&x| x % 2 == 0));
        }
    }

    #[test]
    fn case2() {
        let nums = vec![8, 10, 3, 20, 12, 4, 10, 8, 4, 0, 5, 17, 7, 20, 3];
        let results = Solution::exchange(nums);
        if let Some(p) = results.iter().position(|&x| x % 2 == 0) {
            assert!(results[p..].iter().all(|&x| x % 2 == 0));
        }
    }
}
