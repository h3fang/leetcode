pub struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];
        for i in 1..n + 1 {
            for j in 1..=i {
                if j * j > i {
                    break;
                }
                if !dp[i - j * j] {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::winner_square_game(1));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::winner_square_game(2));
    }

    #[test]
    fn case3() {
        assert_eq!(true, Solution::winner_square_game(4));
    }

    #[test]
    fn case4() {
        assert_eq!(false, Solution::winner_square_game(7));
    }

    #[test]
    fn case5() {
        assert_eq!(true, Solution::winner_square_game(99));
    }
}
