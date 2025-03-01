pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn recursive<'a>(
            s: &'a str,
            i: usize,
            dp: &mut [Vec<bool>],
            curr: &mut Vec<&'a str>,
            result: &mut Vec<Vec<String>>,
        ) {
            if i == s.len() {
                result.push(curr.iter().map(|s| s.to_string()).collect());
                return;
            }

            for j in i..s.len() {
                if dp[i][j] {
                    curr.push(&s[i..=j]);
                    recursive(s, j + 1, dp, curr, result);
                    curr.pop();
                }
            }
        }

        let n = s.len();
        let mut dp = vec![vec![true; n]; n];
        let t = s.as_bytes();
        for i in (0..n).rev() {
            for j in i..n {
                dp[i][j] = t[i] == t[j] && (j - i < 2 || dp[i + 1][j - 1]);
            }
        }
        let mut curr = Vec::new();
        let mut result = Vec::new();
        recursive(&s, 0, &mut dp, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "aab".to_string();
        let expected = [vec!["a", "a", "b"], vec!["aa", "b"]];
        let mut expected = expected
            .iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        expected.sort_unstable();

        let mut result = Solution::partition(s);
        result.sort_unstable();

        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let s = "a".to_string();
        let expected = [vec!["a"]];
        let mut expected = expected
            .iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        expected.sort_unstable();

        let mut result = Solution::partition(s);
        result.sort_unstable();

        assert_eq!(expected, result);
    }
}
