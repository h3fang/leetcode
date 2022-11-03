pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<String, Vec<String>> = HashMap::new();
        for dir in paths {
            let mut parts = dir.split_ascii_whitespace();
            let d = parts.next().unwrap();
            for file in parts {
                let (name, content) = file.split_once('(').unwrap();
                let p = d.to_string() + "/" + name;
                let content = content[..content.len() - 1].to_string();
                m.entry(content).or_default().push(p);
            }
        }
        m.into_values().filter(|g| g.len() > 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let paths = [
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)",
        ]
        .iter()
        .map(|p| p.to_string())
        .collect();
        let mut expected = [
            vec!["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            vec!["root/a/1.txt", "root/c/3.txt"],
        ]
        .iter()
        .map(|g| g.to_vec())
        .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_duplicate(paths);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
