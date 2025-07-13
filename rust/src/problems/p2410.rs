pub struct Solution;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();
        let (mut ans, mut i) = (0, 0);
        for t in trainers {
            if i < players.len() && players[i] <= t {
                ans += 1;
                i += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let players = vec![4, 7, 9];
        let trainers = vec![8, 2, 5, 8];
        assert_eq!(2, Solution::match_players_and_trainers(players, trainers));
    }

    #[test]
    fn case2() {
        let players = vec![1, 1, 1];
        let trainers = vec![10];
        assert_eq!(1, Solution::match_players_and_trainers(players, trainers));
    }
}
