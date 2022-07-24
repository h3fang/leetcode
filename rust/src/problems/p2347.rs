pub struct Solution;

impl Solution {
    pub fn best_hand(mut ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|&s| s == suits[0]) {
            return "Flush".into();
        }
        ranks.sort_unstable();
        if ranks.windows(3).any(|w| w[0] == w[1] && w[0] == w[2]) {
            return "Three of a Kind".into();
        }
        if ranks.windows(2).any(|w| w[0] == w[1]) {
            return "Pair".into();
        }
        "High Card".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ranks = vec![13, 2, 3, 1, 9];
        let suits = ["a", "a", "a", "a", "a"];
        let suits = suits.iter().map(|s| s.chars().next().unwrap()).collect();
        assert_eq!("Flush", Solution::best_hand(ranks, suits));
    }
}
