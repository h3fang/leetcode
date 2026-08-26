pub struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();

        if bytes.iter().filter(|b| **b == b'1').count() < k as usize {
            return String::new();
        }

        let mut ans = s.as_str();
        let (mut min, mut i, mut cnt) = (s.len() + 1, 0, 0);
        for (j, &b) in bytes.iter().enumerate() {
            if b == b'1' {
                cnt += 1;
            }
            while cnt > k || bytes[i] == b'0' {
                if bytes[i] == b'1' {
                    cnt -= 1;
                }
                i += 1;
            }
            if cnt == k {
                let len = j - i + 1;
                let s = &s[i..=j];
                if len < min {
                    min = len;
                    ans = s;
                } else if len == min {
                    ans = ans.min(s);
                }
            }
        }

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "11001",
            Solution::shortest_beautiful_substring("100011001".to_string(), 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "11",
            Solution::shortest_beautiful_substring("1011".to_string(), 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "",
            Solution::shortest_beautiful_substring("000".to_string(), 1)
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "11111",
            Solution::shortest_beautiful_substring("01011101000111110".to_string(), 5)
        );
    }
}
