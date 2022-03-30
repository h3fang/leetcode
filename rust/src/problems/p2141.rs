pub struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut left = 0i64;
        let mut right = batteries.iter().map(|b| *b as i64).sum::<i64>() / n as i64;
        let mut result = 0;
        while left <= right {
            let mid = (left + right) / 2;
            let total: i64 = batteries.iter().map(|b| (*b as i64).min(mid)).sum();
            if total >= mid * n as i64 {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
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
        assert_eq!(4, Solution::max_run_time(2, vec![3, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_run_time(2, vec![1, 1, 1, 1]));
    }
}
