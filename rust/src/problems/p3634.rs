pub struct Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (n, k) = (nums.len(), k as i64);
        let (mut w, mut j) = (0, 1);
        for (i, &x) in nums.iter().enumerate() {
            let max = x as i64 * k;
            while j < n && nums[j] as i64 <= max {
                j += 1;
            }
            w = w.max(j - i);
        }
        (n - w) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_removal(vec![2, 1, 5], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_removal(vec![1, 6, 2, 9], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_removal(vec![4, 6], 2));
    }
}
