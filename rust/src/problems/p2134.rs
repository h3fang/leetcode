pub struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k = nums.iter().filter(|&&x| x == 1).count();
        let mut zeros = nums[..k].iter().filter(|&&x| x == 0).count() as i32;
        let mut result = zeros;
        for (i, &x) in nums.iter().chain(&nums).enumerate().skip(k).take(n) {
            if x == 0 {
                zeros += 1;
            }
            if nums[i - k] == 0 {
                zeros -= 1;
            }
            result = result.min(zeros);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_swaps(vec![1, 1, 0, 0, 1]));
    }
}
