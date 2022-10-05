pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = HashMap::new();
        for cpd in &cpdomains {
            let (c, d) = cpd.split_once(' ').unwrap();
            let c = c.parse::<i32>().unwrap();
            let bytes = d.as_bytes();
            *m.entry(d).or_insert(0) += c;
            for (i, &b) in bytes.iter().enumerate() {
                if b == b'.' {
                    *m.entry(&d[i + 1..]).or_insert(0) += c;
                }
            }
        }
        m.into_iter().map(|(k, v)| format!("{} {}", v, k)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let cpdomains = ["9001 discuss.leetcode.com"]
            .iter()
            .map(|d| d.to_string())
            .collect();
        let mut expected = ["9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"]
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::subdomain_visits(cpdomains);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let cpdomains = [
            "900 google.mail.com",
            "50 yahoo.com",
            "1 intel.mail.com",
            "5 wiki.org",
        ]
        .iter()
        .map(|d| d.to_string())
        .collect();
        let mut expected = [
            "901 mail.com",
            "50 yahoo.com",
            "900 google.mail.com",
            "5 wiki.org",
            "5 org",
            "1 intel.mail.com",
            "951 com",
        ]
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::subdomain_visits(cpdomains);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
