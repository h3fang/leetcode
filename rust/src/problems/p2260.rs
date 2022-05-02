use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut last = HashMap::new();
        let mut min = usize::MAX;
        for (i, &c) in cards.iter().enumerate() {
            if let Some(&prev) = last.get(&c) {
                min = min.min(i - prev as usize + 1);
            }
            last.insert(c, i as i32);
        }
        min as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::minimum_card_pickup(vec![1, 0, 5, 3]));
    }
}
