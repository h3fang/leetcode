pub struct Solution;

fn distance(a: &str, b: &str) -> usize {
    a.bytes().zip(b.bytes()).filter(|(a, b)| a != b).count()
}

fn check(words: &[String], groups: &[i32], i: usize, j: usize) -> bool {
    groups[i] != groups[j]
        && words[i].len() == words[j].len()
        && distance(&words[i], &words[j]) == 1
}

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut f = vec![(1, 0); n];
        for i in 1..n {
            let mut max = (1, 0);
            for (j, e) in f[..i].iter().enumerate() {
                if check(&words, &groups, i, j) && max.0 < e.0 + 1 {
                    max = (e.0 + 1, j);
                }
            }
            f[i] = max;
        }
        let (mut i, (mut max, _)) = f.iter().cloned().enumerate().max_by_key(|e| e.1).unwrap();
        let mut ans = Vec::with_capacity(max);
        while max > 0 {
            ans.push(words[i].to_string());
            i = f[i].1;
            max -= 1;
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["bab", "dab", "cab"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let groups = vec![1, 2, 2];
        let ans = Solution::get_words_in_longest_subsequence(words, groups);
        let expected: Vec<Vec<String>> = [["bab", "cab"], ["bab", "dab"]]
            .iter()
            .map(|v| v.iter().map(|w| w.to_string()).collect())
            .collect();
        assert!(expected.contains(&ans));
    }

    #[test]
    fn case2() {
        let words = ["a", "b", "c", "d"].iter().map(|w| w.to_string()).collect();
        let groups = vec![1, 2, 3, 4];
        let ans = Solution::get_words_in_longest_subsequence(words, groups);
        let expected: Vec<String> = ["a", "b", "c", "d"].iter().map(|w| w.to_string()).collect();
        assert_eq!(expected, ans);
    }
}
