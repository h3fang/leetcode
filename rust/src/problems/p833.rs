pub struct Solution;

impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut ids = (0..indices.len()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| indices[i]);
        let mut prev = 0;
        let mut result = String::new();
        for i in ids {
            let j = indices[i] as usize;
            let src = sources[i].as_str();
            let k = j + src.len();
            if k <= s.len() && &s[j..k] == src {
                result.push_str(&s[prev..j]);
                result.push_str(&targets[i]);
                prev = k;
            }
        }
        result.push_str(&s[prev..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abcd".to_string();
        let indices = vec![0, 2];
        let sources = ["a", "cd"].iter().map(|e| e.to_string()).collect();
        let targets = ["eee", "ffff"].iter().map(|e| e.to_string()).collect();
        assert_eq!(
            "eeebffff",
            Solution::find_replace_string(s, indices, sources, targets)
        );
    }

    #[test]
    fn case2() {
        let s = "abcd".to_string();
        let indices = vec![0, 2];
        let sources = ["ab", "ec"].iter().map(|e| e.to_string()).collect();
        let targets = ["eee", "ffff"].iter().map(|e| e.to_string()).collect();
        assert_eq!(
            "eeecd",
            Solution::find_replace_string(s, indices, sources, targets)
        );
    }
}
