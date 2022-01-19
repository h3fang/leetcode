use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut m = HashSet::new();
        for (i, &n) in nums.iter().enumerate() {
            if i > k {
                m.remove(&nums[i - k - 1]);
            }
            if m.contains(&n) {
                return true;
            }
            m.insert(n);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1)
        );
    }
}
