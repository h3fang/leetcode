pub struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut g = 1;
        for w in word.as_bytes().windows(2) {
            if w[1] <= w[0] {
                g += 1;
            }
        }
        g * 3 - word.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::add_minimum("b".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::add_minimum("aaa".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::add_minimum("abc".to_string()));
    }
}
