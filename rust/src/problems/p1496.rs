pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut s = HashSet::new();
        let mut curr = (0, 0);
        for &b in path.as_bytes() {
            s.insert(curr);
            curr = match b {
                b'N' => (curr.0 - 1, curr.1),
                b'E' => (curr.0, curr.1 + 1),
                b'S' => (curr.0 + 1, curr.1),
                b'W' => (curr.0, curr.1 - 1),
                _ => unreachable!(),
            };
            if s.contains(&curr) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::is_path_crossing("NES".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_path_crossing("NESWW".to_string()));
    }
}
