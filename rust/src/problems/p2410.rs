pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        let mut m = BTreeMap::new();
        for t in trainers {
            *m.entry(t).or_insert(0) += 1;
        }
        for (i, &p) in players.iter().enumerate() {
            if let Some((&t, c)) = m.range_mut(p..).next() {
                *c -= 1;
                if *c == 0 {
                    m.remove(&t);
                }
            } else {
                return i as i32;
            }
        }
        players.len() as i32
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
