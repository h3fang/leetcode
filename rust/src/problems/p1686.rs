pub struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut indices = (0..alice_values.len()).collect::<Vec<_>>();
        indices.sort_unstable_by_key(|&i| alice_values[i] + bob_values[i]);
        let mut diff = 0;
        for (i, j) in indices.into_iter().rev().enumerate() {
            if i % 2 == 0 {
                diff += alice_values[j];
            } else {
                diff -= bob_values[j];
            }
        }
        diff.signum()
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
