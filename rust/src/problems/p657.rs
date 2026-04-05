pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut h, mut v) = (0, 0);
        for &b in moves.as_bytes() {
            match b {
                b'L' => h -= 1,
                b'R' => h += 1,
                b'U' => v += 1,
                b'D' => v -= 1,
                _ => unreachable!(),
            }
        }
        h == 0 && v == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::judge_circle("UD".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::judge_circle("LL".to_string()));
    }
}
