pub struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (mut l, mut curr) = (0, 0);
        for (r, &e) in nums.iter().enumerate() {
            curr += e as i64;
            if (r + 1 - l) as i64 * e as i64 - curr > k as i64 {
                curr -= nums[l] as i64;
                l += 1;
            }
        }
        (nums.len() - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_frequency(vec![1, 2, 4], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_frequency(vec![1, 4, 8, 13], 5));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::max_frequency(vec![3, 9, 6], 2));
    }
}
