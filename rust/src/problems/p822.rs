pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut same = HashSet::new();
        for (&f, &b) in fronts.iter().zip(&backs) {
            if f == b {
                same.insert(f);
            }
        }
        let mut result = i32::MAX;
        for n in fronts.into_iter().chain(backs) {
            if n < result && !same.contains(&n) {
                result = n;
            }
        }
        result % i32::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::flipgame(vec![1], vec![1]));
    }
}
