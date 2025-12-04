pub struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        directions
            .trim_start_matches('L')
            .trim_end_matches('R')
            .as_bytes()
            .iter()
            .filter(|&&b| b != b'S')
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_collisions("RLRSLL".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_collisions("LLRR".to_string()));
    }
}
