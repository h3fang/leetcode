pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        match n.cmp(&d) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => job_difficulty.iter().sum(),
            std::cmp::Ordering::Greater => {
                let mut dp = vec![vec![i32::MAX / 2; d + 1]; n];
                dp[0][1] = job_difficulty[0];
                for i in 1..n {
                    let mut max = vec![0; i + 1];
                    max[i] = job_difficulty[i];
                    for j in (0..i).rev() {
                        max[j] = max[j + 1].max(job_difficulty[j]);
                    }
                    for j in 2..=d.min(i + 1) {
                        for k in 0..i {
                            dp[i][j] = dp[i][j].min(dp[k][j - 1] + max[k + 1]);
                        }
                    }
                    dp[i][1] = max[0];
                }
                dp[n - 1][d]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_difficulty(vec![9, 9, 9], 4));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::min_difficulty(vec![1, 1, 1], 3));
    }
}
