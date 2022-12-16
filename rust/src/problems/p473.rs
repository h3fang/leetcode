pub struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            return false;
        }
        let side = sum / 4;
        if matchsticks.iter().any(|&m| m > side) {
            return false;
        }
        let n_states = 1 << matchsticks.len();
        let mut dp = vec![-1; n_states];
        dp[0] = 0;

        for s in 1..n_states {
            for (i, m) in matchsticks.iter().enumerate() {
                if s & (1 << i) == 0 {
                    continue;
                }
                let s1 = s & !(1 << i);
                if dp[s1] >= 0 && dp[s1] + m <= side {
                    dp[s] = (dp[s1] + m) % side;
                    break;
                }
            }
        }
        dp[n_states - 1] == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::makesquare(vec![1, 1, 2, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::makesquare(vec![3, 3, 3, 3, 4]));
    }
}
