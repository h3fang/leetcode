use std::collections::{btree_map::Entry, BTreeMap};

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() as i32 % group_size != 0 {
            return false;
        }
        let mut m = BTreeMap::new();
        for n in hand {
            *m.entry(n).or_insert(0i32) += 1;
        }
        while let Some((&first, &count)) = m.iter().next() {
            if count > 0 {
                for key in first..first + group_size {
                    match m.entry(key) {
                        Entry::Vacant(_) => return false,
                        Entry::Occupied(mut e) => {
                            if *e.get() < count {
                                return false;
                            } else {
                                *e.get_mut() -= count;
                            }
                        }
                    }
                }
            }
            m.remove(&first);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
        )
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4))
    }
}
