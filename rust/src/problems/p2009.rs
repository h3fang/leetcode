pub struct Solution;

fn unique(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());
    let mut prev = nums[0];
    result.push(prev);
    for &n in nums.iter().skip(1) {
        if n != prev {
            result.push(n);
            prev = n;
        }
    }
    result
}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len() as i32;
        let nums = unique(&nums);
        let mut result = 0;
        for (r, &b) in nums.iter().enumerate() {
            let l = nums[..r].partition_point(|&a| a < b - n + 1);
            result = result.max(r - l + 1);
        }
        n - result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::min_operations(vec![4, 2, 5, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_operations(vec![1, 2, 3, 5, 6]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::min_operations(vec![1, 10, 100, 1000]));
    }
}
