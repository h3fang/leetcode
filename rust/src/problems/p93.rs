pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn bt(s: &str, curr: &mut Vec<u8>, result: &mut Vec<String>) {
            if curr.len() == 4 {
                if s.is_empty() {
                    result.push(format!("{}.{}.{}.{}", curr[0], curr[1], curr[2], curr[3]));
                }
                return;
            }
            for i in 1..=s.len() {
                if let Ok(n) = s[..i].parse::<u8>() {
                    if n.to_string().len() != i {
                        continue;
                    }
                    curr.push(n);
                    bt(&s[i..], curr, result);
                    curr.pop();
                }
            }
        }

        let mut curr = Vec::new();
        let mut result = Vec::new();
        bt(&s, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "25525511135".to_string();
        let expected = ["255.255.11.135", "255.255.111.35"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::restore_ip_addresses(s));
    }

    #[test]
    fn case2() {
        let s = "0000".to_string();
        let expected = ["0.0.0.0"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::restore_ip_addresses(s));
    }

    #[test]
    fn case3() {
        let s = "101023".to_string();
        let expected = [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::restore_ip_addresses(s));
    }

    #[test]
    fn case4() {
        let s = "1111".to_string();
        let expected = ["1.1.1.1"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::restore_ip_addresses(s));
    }

    #[test]
    fn case5() {
        let s = "010010".to_string();
        let expected = ["0.10.0.10", "0.100.1.0"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::restore_ip_addresses(s));
    }
}
