pub struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut max, mut min, mut sum) = (0, 0, 0);
        for n in nums {
            sum += n;
            max = max.max(sum);
            min = min.min(sum);
        }
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]));
    }
}
