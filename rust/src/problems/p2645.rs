pub struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let (mut last, mut result) = (b'c', 0);
        for &b in word.as_bytes() {
            let d = (b + 3 - last) % 3;
            match d {
                0 => result += 2,
                2 => result += 1,
                _ => {}
            }
            last = b;
        }
        result + (b'c' - last) as i32
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
