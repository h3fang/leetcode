pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut ss = vec![0; n];
        ss[n - 1] = stone_value[n - 1];
        for i in (0..n - 1).rev() {
            ss[i] = ss[i + 1] + stone_value[i];
        }
        let mut f = vec![0; n + 1];
        for i in (0..n).rev() {
            let mut best = f[i + 1];
            for (j, &e) in f.iter().enumerate().skip(i + 2) {
                if j > i + 3 {
                    break;
                }
                best = best.min(e);
            }
            f[i] = ss[i] - best;
        }
        let total = stone_value.iter().sum::<i32>();
        match (f[0] * 2).cmp(&total) {
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
