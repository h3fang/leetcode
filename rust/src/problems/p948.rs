pub struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort_unstable();
        let mut result = 0;
        let mut l = 0;
        let mut r = tokens.len() as i32 - 1;
        let mut points = 0;
        while l <= r && (power >= tokens[l as usize] || points > 0) {
            while l <= r && power >= tokens[l as usize] {
                power -= tokens[l as usize];
                l += 1;
                points += 1;
            }
            result = result.max(points);
            if l <= r && points > 0 {
                power += tokens[r as usize];
                r -= 1;
                points -= 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::bag_of_tokens_score(vec![100], 50));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::bag_of_tokens_score(vec![100, 200], 150));
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200)
        );
    }
}
