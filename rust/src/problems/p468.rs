pub struct Solution;

impl Solution {
    pub fn valid_ip_address(q: String) -> String {
        if q.contains('.') {
            let parts = q.split('.').collect::<Vec<_>>();
            if parts.len() != 4 {
                return "Neither".to_string();
            }
            for p in parts {
                if let Ok(n) = p.parse::<u8>() {
                    if (n == 0 && p.len() != 1) || (n > 0 && p.as_bytes()[0] == b'0') {
                        return "Neither".to_string();
                    }
                } else {
                    return "Neither".to_string();
                }
            }
            "IPv4".to_string()
        } else {
            let parts = q.split(':').collect::<Vec<_>>();
            if parts.len() != 8 {
                return "Neither".to_string();
            }
            for p in parts {
                if p.len() > 4 || u16::from_str_radix(p, 16).is_err() {
                    return "Neither".to_string();
                }
            }
            "IPv6".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("IPv4", Solution::valid_ip_address("172.16.254.1".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "IPv6",
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".into())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "Neither",
            Solution::valid_ip_address("256.256.256.256".into())
        );
    }
}
