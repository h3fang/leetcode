pub struct Solution;

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut result = 0;
        while l <= r {
            if l == r {
                result += nums[l] as i64;
                break;
            } else {
                let (a, b) = (nums[l] as i64, nums[r] as i64);
                result += a * 10i64.pow(b.ilog10() + 1) + b;
                l += 1;
                r -= 1;
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
        assert_eq!(596, Solution::find_the_array_conc_val(vec![7, 52, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            673,
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12])
        );
    }
}
