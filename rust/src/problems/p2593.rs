pub struct Solution;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let (mut i, mut result, n) = (0, 0, nums.len());
        while i < n {
            let mut j = i;
            while j + 1 < n && nums[j] > nums[j + 1] {
                j += 1;
            }
            let mut k = j as i64;
            while k >= i as i64 {
                result += nums[k as usize] as i64;
                k -= 2;
            }
            i = j + 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::find_score(vec![2, 1, 3, 4, 5, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::find_score(vec![2, 3, 5, 1, 3, 2]));
    }
}
