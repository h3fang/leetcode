pub struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut pre = vec![0; n + 1];
        let mut result: i64 = 0;
        for j in 0..n {
            let mut suf = 0;
            for k in (j + 1..n).rev() {
                if nums[j] > nums[k] {
                    result += pre[nums[k] as usize] as i64 * suf as i64;
                } else {
                    suf += 1;
                }
            }
            for x in pre.iter_mut().take(n + 1).skip((nums[j] + 1) as usize) {
                *x += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_quadruplets(vec![1, 3, 2, 4, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_quadruplets(vec![1, 2, 3, 4]));
    }
}
