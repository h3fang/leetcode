pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn signature(s: &str) -> [i32; 26] {
            let mut f = [0; 26];
            for b in s.as_bytes() {
                f[(b - b'a') as usize] += 1;
            }
            f
        }

        let mut m: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let f = signature(&s);
            m.entry(f).or_default().push(s);
        }
        m.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut expected = [vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
            .iter()
            .map(|g| g.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        expected.iter_mut().for_each(|g| g.sort_unstable());
        expected.sort_unstable();

        let mut result = Solution::group_anagrams(strs);
        result.iter_mut().for_each(|g| g.sort_unstable());
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
