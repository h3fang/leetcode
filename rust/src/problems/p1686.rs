pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut q = alice_values
            .into_iter()
            .zip(bob_values)
            .map(|(a, b)| (a + b, a, b))
            .collect::<BinaryHeap<_>>();
        let (mut a, mut b, mut i) = (0, 0, 0);
        while let Some((_, x, y)) = q.pop() {
            if i % 2 == 0 {
                a += x;
            } else {
                b += y;
            }
            i += 1;
        }
        match a.cmp(&b) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::stone_game_vi(vec![1, 3], vec![2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::stone_game_vi(vec![1, 2], vec![3, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::stone_game_vi(vec![2, 4, 3], vec![1, 6, 7]));
    }
}
