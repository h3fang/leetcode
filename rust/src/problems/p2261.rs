use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let valid = nums
            .iter()
            .map(|&n| if n % p == 0 { 1 } else { 0 })
            .collect::<Vec<_>>();
        let mut s = HashSet::new();
        for left in 0..nums.len() {
            let mut c = 0;
            for right in left..nums.len() {
                c += valid[right];
                if c > k {
                    break;
                }
                s.insert(nums[left..=right].to_vec());
            }
        }
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(11, Solution::count_distinct(vec![2, 3, 3, 2, 2], 2, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_distinct(vec![1, 2, 3, 4], 4, 1));
    }
}
