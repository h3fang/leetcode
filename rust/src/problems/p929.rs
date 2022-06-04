use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for e in emails {
            let (local, domain) = e.split_once('@').unwrap();
            let local = if let Some(p) = local.find('+') {
                &local[..p]
            } else {
                local
            };
            let e = local.replace('.', "") + "@" + domain;
            set.insert(e);
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let emails = [
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com",
        ];
        let emails = emails.iter().map(|e| e.to_string()).collect();
        assert_eq!(2, Solution::num_unique_emails(emails));
    }
}
