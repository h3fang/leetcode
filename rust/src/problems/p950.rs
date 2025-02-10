pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut result = VecDeque::with_capacity(deck.len());
        for (i, x) in deck.into_iter().rev().enumerate() {
            if i > 1 {
                let y = result.pop_back().unwrap();
                result.push_front(y);
            }
            result.push_front(x);
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, 13, 3, 11, 5, 17, 7],
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 1000],
            Solution::deck_revealed_increasing(vec![1, 1000])
        );
    }
}
