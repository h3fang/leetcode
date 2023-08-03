pub struct Solution;

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut in_block_comment = false;
        let mut result = vec![];
        let mut t = String::new();
        for s in source {
            let s = s.as_bytes();
            let m = s.len();
            let mut j = 0;
            while j < m {
                if in_block_comment {
                    if j + 1 < m && s[j] == b'*' && s[j + 1] == b'/' {
                        in_block_comment = false;
                        j += 2;
                    } else {
                        j += 1;
                    }
                } else if j + 1 < m && s[j] == b'/' && s[j + 1] == b'/' {
                    break;
                } else if j + 1 < m && s[j] == b'/' && s[j + 1] == b'*' {
                    in_block_comment = true;
                    j += 2;
                } else {
                    t.push(s[j] as char);
                    j += 1;
                }
            }
            if !in_block_comment && !t.is_empty() {
                result.push(std::mem::take(&mut t));
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
        let source = [
            "/*Test program */",
            "int main()",
            "{ ",
            "  // variable declaration ",
            "int a, b, c;",
            "/* This is a test",
            "   multiline  ",
            "   comment for ",
            "   testing */",
            "a = b + c;",
            "}",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let expected = ["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::remove_comments(source));
    }

    #[test]
    fn case2() {
        let source = ["a/*comment", "line", "more_comment*/b"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        let expected = ["ab"].iter().map(|l| l.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::remove_comments(source));
    }
}
