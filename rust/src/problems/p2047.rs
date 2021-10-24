pub struct Solution;

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        sentence
            .split_ascii_whitespace()
            .filter(|&t| {
                let mut has_hypen = false;
                for (i, c) in t.chars().enumerate() {
                    match c {
                        '-' => {
                            if has_hypen
                                || i == 0
                                || i == t.len() - 1
                                || !t.chars().nth(i + 1).unwrap().is_ascii_lowercase()
                            {
                                return false;
                            } else {
                                has_hypen = true;
                            }
                        }
                        '!' => return i == t.len() - 1,
                        '.' => return i == t.len() - 1,
                        ',' => return i == t.len() - 1,
                        c if c.is_ascii_lowercase() => {}
                        _ => return false,
                    }
                }
                true
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let sentence = "cat and  dog".to_string();
        assert_eq!(3, Solution::count_valid_words(sentence));
    }

    #[test]
    fn case2() {
        let sentence = "!this  1-s b8d!".to_string();
        assert_eq!(0, Solution::count_valid_words(sentence));
    }

    #[test]
    fn case3() {
        let sentence = "alice and  bob are playing stone-game10".to_string();
        assert_eq!(5, Solution::count_valid_words(sentence));
    }

    #[test]
    fn case4() {
        let sentence = "he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.".to_string();
        assert_eq!(6, Solution::count_valid_words(sentence));
    }
}
