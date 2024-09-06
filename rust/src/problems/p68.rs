pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut i = 0;
        let mut row = Vec::new();
        let mut row_width = 0;
        let mut ans = Vec::new();
        while i < words.len() {
            if row_width == 0 || row_width + 1 + words[i].len() <= max_width as usize {
                row.push(words[i].as_str());
                row_width += usize::from(row_width != 0);
                row_width += words[i].len();
                i += 1;
            } else {
                let slots = row.len() - 1;
                let spaces = max_width as usize - row_width;
                if slots == 0 {
                    ans.push(row[0].to_string() + &" ".repeat(spaces));
                } else {
                    let right = spaces / slots + 1;
                    let left = spaces % slots;
                    let mut s = row[..left]
                        .iter()
                        .map(|s| s.to_string() + &" ".repeat(right + 1))
                        .collect();
                    let spaces = &" ".repeat(right);
                    s += row[left..].join(spaces).as_str();
                    ans.push(s);
                }
                row.clear();
                row_width = 0;
            }
        }
        if !row.is_empty() {
            let mut s = row.join(" ");
            while s.len() < max_width as usize {
                s.push(' ');
            }
            ans.push(s);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = [
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ];
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            vec!["This    is    an", "example  of text", "justification.  "],
            Solution::full_justify(words, 16)
        );
    }

    #[test]
    fn case2() {
        let words = ["What", "must", "be", "acknowledgment", "shall", "be"];
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            vec!["What   must   be", "acknowledgment  ", "shall be        "],
            Solution::full_justify(words, 16)
        );
    }

    #[test]
    fn case3() {
        let words = vec![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ];
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ],
            Solution::full_justify(words, 20)
        );
    }
}
