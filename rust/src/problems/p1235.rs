pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs = start_time
            .into_iter()
            .zip(end_time.into_iter().zip(profit.into_iter()))
            .map(|(a, (b, c))| (a, b, c))
            .collect::<Vec<_>>();
        jobs.sort_unstable_by_key(|e| e.1);
        let mut dp = vec![0; n + 1];

        let binary_search = |mut right: usize, target: i32| {
            let mut left = 0;
            while left < right {
                let mid = left + (right - left) / 2;
                if jobs[mid].1 > target {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        };

        for i in 1..=n {
            let k = binary_search(i - 1, jobs[i - 1].0);
            dp[i] = dp[i - 1].max(dp[k] + jobs[i - 1].2);
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        assert_eq!(120, Solution::job_scheduling(start_time, end_time, profit));
    }

    #[test]
    fn case2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        assert_eq!(150, Solution::job_scheduling(start_time, end_time, profit));
    }

    #[test]
    fn case3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        assert_eq!(6, Solution::job_scheduling(start_time, end_time, profit));
    }
}
