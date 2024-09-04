pub struct Solution;

impl Solution {
    pub fn count_ways(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = i32::from(nums[0] > 0);
        for (i, &x) in nums.iter().enumerate().skip(1) {
            if nums[i - 1] < i as i32 && x > i as i32 {
                result += 1;
            }
        }
        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_ways(vec![1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::count_ways(vec![1, 1, 0, 1]));
    }
}
