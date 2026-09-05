pub struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut suf = vec![i32::MAX; n + 1];
        for (i, &x) in nums.iter().enumerate().rev() {
            suf[i] = suf[i + 1].min(x);
        }

        let mut max = i32::MIN;

        for (i, x) in nums.into_iter().enumerate() {
            max = max.max(x);
            if max - suf[i] <= k {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::first_stable_index(vec![5, 0, 1, 4], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::first_stable_index(vec![3, 2, 1], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::first_stable_index(vec![0], 0));
    }
}
