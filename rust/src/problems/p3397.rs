pub struct Solution;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut min = i32::MIN;
        let mut ans = 0;
        for x in nums {
            if min > x + k {
                continue;
            }
            min = min.max(x - k) + 1;
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            6,
            Solution::max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_distinct_elements(vec![4, 4, 4, 4], 1));
    }
}
