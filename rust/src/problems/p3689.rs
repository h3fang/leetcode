pub struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        for x in nums {
            min = min.min(x);
            max = max.max(x);
        }
        (i64::from(max) - i64::from(min)) * i64::from(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_total_value(vec![1, 3, 2], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::max_total_value(vec![4, 2, 5, 1], 3));
    }
}
