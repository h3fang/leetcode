pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut starts = HashSet::new();
        for p in &paths {
            starts.insert(p[0].as_str());
        }
        paths
            .iter()
            .find(|p| !starts.contains(p[1].as_str()))
            .map(|p| p[1].clone())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let paths = [
            ["London", "New York"],
            ["New York", "Lima"],
            ["Lima", "Sao Paulo"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.to_string()).collect())
        .collect();
        assert_eq!("Sao Paulo", Solution::dest_city(paths));
    }

    #[test]
    fn case2() {
        let paths = [["B", "C"], ["D", "B"], ["C", "A"]]
            .iter()
            .map(|r| r.iter().map(|c| c.to_string()).collect())
            .collect();
        assert_eq!("A", Solution::dest_city(paths));
    }

    #[test]
    fn case3() {
        let paths = [["A", "Z"]]
            .iter()
            .map(|r| r.iter().map(|c| c.to_string()).collect())
            .collect();
        assert_eq!("Z", Solution::dest_city(paths));
    }
}
