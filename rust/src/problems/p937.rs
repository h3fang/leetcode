pub struct Solution;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let (mut alphabetic, digital): (Vec<String>, Vec<String>) = logs
            .into_iter()
            .partition(|l| l.ends_with(|c: char| c.is_alphabetic()));

        alphabetic.sort_unstable_by(|a, b| {
            let (tag_a, a) = a.split_once(' ').unwrap();
            let (tag_b, b) = b.split_once(' ').unwrap();
            match a.cmp(b) {
                std::cmp::Ordering::Equal => tag_a.cmp(tag_b),
                x => x,
            }
        });

        alphabetic.extend(digital);
        alphabetic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_logs(logs: &[&str]) -> Vec<String> {
        logs.iter().map(|l| l.to_string()).collect()
    }

    #[test]
    fn case1() {
        let logs = parse_logs(&[
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero",
        ]);
        let expected = parse_logs(&[
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6",
        ]);
        assert_eq!(expected, Solution::reorder_log_files(logs));
    }

    #[test]
    fn case2() {
        let logs = parse_logs(&[
            "a1 9 2 3 1",
            "g1 act car",
            "zo4 4 7",
            "ab1 off key dog",
            "a8 act zoo",
        ]);
        let expected = parse_logs(&[
            "g1 act car",
            "a8 act zoo",
            "ab1 off key dog",
            "a1 9 2 3 1",
            "zo4 4 7",
        ]);
        assert_eq!(expected, Solution::reorder_log_files(logs));
    }
}
