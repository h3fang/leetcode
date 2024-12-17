pub struct Solution;

fn partial_match_table(s: &[u8]) -> Vec<usize> {
    let mut r = Vec::with_capacity(s.len());
    r.push(0);
    let mut max = 0;
    for &b in s.iter().skip(1) {
        while max > 0 && s[max] != b {
            max = r[max - 1];
        }
        if s[max] == b {
            max += 1;
        }
        r.push(max);
    }
    r
}

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let mut back = vec![0; n];
        for w in words {
            let s = format!("{w}#{target}");
            let t = partial_match_table(s.as_bytes());
            let m = w.len();
            for i in 0..n {
                back[i] = back[i].max(t[m + 1 + i]);
            }
        }
        let mut f = vec![i32::MAX / 2; n + 1];
        f[0] = 0;
        for i in 1..=n {
            f[i] = f[i - back[i - 1]] + 1;
            if f[i] > n as i32 {
                return -1;
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abc", "aaaaa", "bcdef"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(
            3,
            Solution::min_valid_strings(words, "aabcdabc".to_string())
        );
    }

    #[test]
    fn case2() {
        let words = ["abababab", "ab"].iter().map(|w| w.to_string()).collect();
        assert_eq!(
            2,
            Solution::min_valid_strings(words, "ababaababa".to_string())
        );
    }

    #[test]
    fn case3() {
        let words = ["abcdef"].iter().map(|w| w.to_string()).collect();
        assert_eq!(-1, Solution::min_valid_strings(words, "xyz".to_string()));
    }
}
