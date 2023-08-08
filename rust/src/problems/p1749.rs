pub struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut max, mut min) = (0, 0);
        let (mut p_sum, mut n_sum) = (0, 0);
        for n in nums {
            p_sum += n;
            max = max.max(p_sum);
            p_sum = p_sum.max(0);

            n_sum += n;
            min = min.min(n_sum);
            n_sum = n_sum.min(0);
        }
        max.max(-min)
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
