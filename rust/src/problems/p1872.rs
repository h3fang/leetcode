pub struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut sum: i32 = stones[..n - 1].iter().sum();
        let mut f = sum + stones[n - 1];
        for &x in stones[1..n - 1].iter().rev() {
            f = f.max(sum - f);
            sum -= x;
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::stone_game_viii(vec![-1, 2, -3, 4, -5]));
    }

    #[test]
    fn case2() {
        assert_eq!(13, Solution::stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]));
    }

    #[test]
    fn case3() {
        assert_eq!(-22, Solution::stone_game_viii(vec![-10, -12]));
    }
}
