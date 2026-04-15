pub struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let (n, start_index) = (words.len(), start_index as usize);
        for d in 0..=n / 2 {
            if words[(start_index + d) % n] == target || words[(start_index + n - d) % n] == target
            {
                return d as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["hello", "i", "am", "leetcode", "hello"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(1, Solution::closest_target(words, "hello".to_string(), 1));
    }

    #[test]
    fn case2() {
        let words = ["a", "b", "leetcode"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(
            1,
            Solution::closest_target(words, "leetcode".to_string(), 0)
        );
    }

    #[test]
    fn case3() {
        let words = ["i", "eat", "leetcode"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(-1, Solution::closest_target(words, "ate".to_string(), 0));
    }
}
