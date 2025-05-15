pub struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::with_capacity(words.len());
        let mut bit = 2;
        for (w, g) in words.into_iter().zip(groups) {
            if g == bit {
                continue;
            }
            ans.push(w.to_string());
            bit = g;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["e", "a", "b"].iter().map(|w| w.to_string()).collect();
        let groups = vec![0, 0, 1];
        let ans = Solution::get_longest_subsequence(words, groups);
        let expected: Vec<Vec<String>> = [["e", "b"], ["a", "b"]]
            .iter()
            .map(|v| v.iter().map(|w| w.to_string()).collect())
            .collect();
        assert!(expected.contains(&ans));
    }

    #[test]
    fn case2() {
        let words = ["a", "b", "c", "d"].iter().map(|w| w.to_string()).collect();
        let groups = vec![1, 0, 1, 1];
        let ans = Solution::get_longest_subsequence(words, groups);
        let expected: Vec<Vec<String>> = [["a", "b", "c"], ["a", "b", "d"]]
            .iter()
            .map(|v| v.iter().map(|w| w.to_string()).collect())
            .collect();
        assert!(expected.contains(&ans));
    }
}
