pub struct Solution;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let n = text.len();
        let mut r = String::with_capacity(n);
        let mut i = 0;
        while i < n {
            match text.as_bytes()[i] {
                b'&' => {
                    let mut matched = false;
                    for (pattern, c) in [
                        ("quot;", '"'),
                        ("apos;", '\''),
                        ("amp;", '&'),
                        ("gt;", '>'),
                        ("lt;", '<'),
                        ("frasl;", '/'),
                    ] {
                        if text[i + 1..].starts_with(pattern) {
                            r.push(c);
                            i += pattern.len();
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        r.push('&');
                    }
                }
                b => r.push(b as char),
            }
            i += 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "& is an HTML entity but &ambassador; is not.",
            Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "and I quote: \"...\"",
            Solution::entity_parser("and I quote: &quot;...&quot;".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "Stay home! Practice on Leetcode :)",
            Solution::entity_parser("Stay home! Practice on Leetcode :)".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "x > y && x < y is always false",
            Solution::entity_parser("x &gt; y &amp;&amp; x &lt; y is always false".to_string())
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            "leetcode.com/problemset/all",
            Solution::entity_parser("leetcode.com&frasl;problemset&frasl;all".to_string())
        );
    }
}
