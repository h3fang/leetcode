pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0i64; n + 1];
        for i in (0..n).rev() {
            dp[i] =
                dp[i + 1].max(questions[i][0] as i64 + dp[n.min(i + questions[i][1] as usize + 1)]);
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let questions = [[3, 2], [4, 3], [4, 4], [2, 5]];
        let questions = questions.iter().map(|q| q.to_vec()).collect();
        assert_eq!(5, Solution::most_points(questions));
    }

    #[test]
    fn case2() {
        let questions = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]];
        let questions = questions.iter().map(|q| q.to_vec()).collect();
        assert_eq!(7, Solution::most_points(questions));
    }
}
