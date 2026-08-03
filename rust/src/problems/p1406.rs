pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut suf = 0;
        let mut f = (0, 0, 0);
        for &s in stone_value.iter().rev() {
            suf += s;
            f = (suf - f.0.min(f.1).min(f.2), f.0, f.1);
        }

        match (2 * f.0).cmp(&suf) {
            std::cmp::Ordering::Less => "Bob".to_string(),
            std::cmp::Ordering::Equal => "Tie".to_string(),
            std::cmp::Ordering::Greater => "Alice".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("Bob", Solution::stone_game_iii(vec![1, 2, 3, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!("Alice", Solution::stone_game_iii(vec![1, 2, 3, -9]));
    }

    #[test]
    fn case3() {
        assert_eq!("Tie", Solution::stone_game_iii(vec![1, 2, 3, 6]));
    }
}
