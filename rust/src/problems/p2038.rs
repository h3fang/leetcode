pub struct Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut prev = b'c';
        let mut curr = 0;
        let mut moves = [0, 0];
        for c in colors.as_bytes() {
            if *c == prev {
                curr += 1;
            } else {
                if prev == b'A' {
                    moves[0] += (curr - 2).max(0);
                } else if prev == b'B' {
                    moves[1] += (curr - 2).max(0);
                }
                prev = *c;
                curr = 1;
            }
        }

        if prev == b'A' {
            moves[0] += (curr - 2).max(0);
        } else if prev == b'B' {
            moves[1] += (curr - 2).max(0);
        }

        moves[0] > moves[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::winner_of_game("AAABABB".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::winner_of_game("AA".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::winner_of_game("AAAABBBB".to_string()));
    }
}
