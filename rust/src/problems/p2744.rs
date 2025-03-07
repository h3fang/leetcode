pub struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        for (i, a) in words.iter().enumerate() {
            for b in &words[i + 1..] {
                if *a == b.chars().rev().collect::<String>() {
                    result += 1;
                }
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
        let words = ["cd", "ac", "dc", "ca", "zz"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(2, Solution::maximum_number_of_string_pairs(words));
    }

    #[test]
    fn case2() {
        let words = ["ab", "ba", "cc"].iter().map(|w| w.to_string()).collect();
        assert_eq!(1, Solution::maximum_number_of_string_pairs(words));
    }

    #[test]
    fn case3() {
        let words = ["aa", "ab"].iter().map(|w| w.to_string()).collect();
        assert_eq!(0, Solution::maximum_number_of_string_pairs(words));
    }
}
