pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let n = nums.len() as i64;
        let (mut l, mut r, mut result) = (n, n, 0);
        for (j, &x) in nums.iter().enumerate() {
            while r > 0 && nums[r as usize - 1] + x > upper {
                r -= 1;
            }
            while l > 0 && nums[l as usize - 1] + x >= lower {
                l -= 1;
            }
            result += r.min(j as i64) - l.min(j as i64);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11));
    }
}
