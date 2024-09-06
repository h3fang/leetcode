pub struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn dfs(num: &str, pprev: Option<usize>, prev: Option<usize>) -> bool {
            if let Some(pprev) = pprev {
                let prev = prev.unwrap();
                let sum = prev + pprev;
                if let Some(remaining) = num.strip_prefix(&sum.to_string()) {
                    if remaining.is_empty() {
                        true
                    } else {
                        dfs(remaining, Some(prev), Some(sum))
                    }
                } else {
                    false
                }
            } else {
                for i in 0..num.len() {
                    let n = num[..=i].parse::<usize>().unwrap();
                    if dfs(&num[i + 1..], prev, Some(n)) {
                        return true;
                    }
                    if i == 0 && num.as_bytes()[0] == b'0' {
                        break;
                    }
                }
                false
            }
        }

        dfs(&num, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_additive_number("112358".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_additive_number("199100199".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_additive_number("0".to_string()));
    }

    #[test]
    fn case4() {
        assert!(!Solution::is_additive_number("10".to_string()));
    }

    #[test]
    fn case5() {
        assert!(!Solution::is_additive_number("1023".to_string()));
    }

    #[test]
    fn case6() {
        assert!(Solution::is_additive_number("101".to_string()));
    }
}
