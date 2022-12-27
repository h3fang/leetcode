pub struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut lo = 1;
        let mut hi = *nums.iter().max().unwrap();
        let mut result = 0;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let mut ops = 0;
            for &num in nums.iter() {
                ops += (num - 1) / mid;
            }
            if ops <= max_operations {
                result = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
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
        assert_eq!(3, Solution::minimum_size(vec![9], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_size(vec![2, 4, 8, 2], 4));
    }
}
