pub struct Solution;

impl Solution {
    pub fn stone_game(_piles: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::stone_game(vec![5, 3, 4, 5]));
    }

    #[test]
    fn case2() {
        assert!(Solution::stone_game(vec![3, 7, 2, 3]));
    }
}
