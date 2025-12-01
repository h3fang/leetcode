pub struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        let mut n = n as i64;
        let mut sum: i64 = batteries.iter().map(|b| *b as i64).sum();
        batteries.sort_unstable();
        for b in batteries.into_iter().rev() {
            if b as i64 <= sum / n {
                return sum / n;
            }
            sum -= b as i64;
            n -= 1;
        }
        unreachable!()
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
