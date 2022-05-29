use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut m: HashMap<String, usize> = HashMap::new();
        for (msg, sender) in messages.into_iter().zip(senders) {
            *m.entry(sender).or_default() += msg.split_ascii_whitespace().count();
        }
        let mut max = 0;
        let mut result = String::new();
        for (k, v) in m {
            if v > max {
                max = v;
                result = k;
            } else if v == max && k > result {
                result = k;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_strings(ss: &[&str]) -> Vec<String> {
        ss.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn case1() {
        let messages = parse_strings(&[
            "Hello userTwooo",
            "Hi userThree",
            "Wonderful day Alice",
            "Nice day userThree",
        ]);
        let senders = parse_strings(&["Alice", "userTwo", "userThree", "Alice"]);
        assert_eq!("Alice", Solution::largest_word_count(messages, senders));
    }

    #[test]
    fn case2() {
        let messages = parse_strings(&[
            "How is leetcode for everyone",
            "Leetcode is useful for practice",
        ]);
        let senders = parse_strings(&["Bob", "Charlie"]);
        assert_eq!("Charlie", Solution::largest_word_count(messages, senders));
    }
}
