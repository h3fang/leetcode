pub struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let d = f64::from(21 * 21 - 8 * n).sqrt().ceil() as i32;
        ((21 - d) / 2) % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_alice_win(12));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_alice_win(1));
    }
}
