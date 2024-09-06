pub struct Solution;

impl Solution {
    pub fn is_valid(code: String) -> bool {
        let c = code.as_bytes();
        let n = c.len();
        let mut tags = vec![];
        let mut i = 0;
        while i < code.len() {
            if c[i] == b'<' {
                if i + 1 == c.len() {
                    return false;
                }
                if c[i + 1] == b'!' {
                    // CDATA
                    if tags.is_empty() {
                        return false;
                    }
                    if i + 9 > n || &code[i + 2..i + 9] != "[CDATA[" {
                        return false;
                    }
                    match code[i + 9..].find("]]>") {
                        Some(j) => {
                            i = i + 9 + j + 3;
                        }
                        None => return false,
                    }
                } else if c[i + 1] == b'/' {
                    // closing tag
                    match code[i + 2..].find('>') {
                        Some(j) => {
                            let tag = &code[i + 2..i + 2 + j];
                            if tags.is_empty() || *tags.last().unwrap() != tag {
                                return false;
                            }
                            tags.pop();
                            i = i + 2 + j + 1;
                            if tags.is_empty() && i != code.len() {
                                return false;
                            }
                        }
                        None => return false,
                    }
                } else {
                    // starting tag
                    match code[i + 1..].find('>') {
                        Some(j) => {
                            let tag = &code[i + 1..i + 1 + j];
                            if tag.is_empty()
                                || tag.len() > 9
                                || !tag.chars().all(|c| c.is_ascii_uppercase())
                            {
                                return false;
                            }
                            tags.push(tag);
                            i = i + 1 + j + 1;
                        }
                        None => return false,
                    }
                }
            } else {
                if tags.is_empty() {
                    return false;
                }
                i += 1;
            }
        }
        tags.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_valid(
            "<DIV>This is the first line <![CDATA[<div>]]></DIV>".into()
        ));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_valid(
            "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".into()
        ));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_valid("<A>  <B> </A>   </B>".into()));
    }

    #[test]
    fn case4() {
        assert!(!Solution::is_valid("<A><!A></A>".into()));
    }
}
