pub struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let mut max = 0;
        let mut min = 0;
        let mut result = vec![0; n + 1];
        for (i, c) in s.char_indices() {
            match c {
                'I' => {
                    max += 1;
                    result[i + 1] = max;
                }
                _ => {
                    min -= 1;
                    result[i + 1] = min;
                }
            }
        }
        result.iter_mut().for_each(|n| *n -= min);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str, arr: &[i32]) -> bool {
        s.len() == arr.len() - 1
            && arr.iter().all(|&n| n >= 0 && n <= arr.len() as i32)
            && arr
                .windows(2)
                .zip(s.chars())
                .all(|(w, c)| (w[0] < w[1] && c == 'I') || (w[0] > w[1] && c == 'D'))
    }

    #[test]
    fn case1() {
        let s = "IDID";
        let arr = Solution::di_string_match(s.into());
        assert!(is_valid(s, &arr));
    }

    #[test]
    fn case2() {
        let s = "III";
        let arr = Solution::di_string_match(s.into());
        assert!(is_valid(s, &arr));
    }

    #[test]
    fn case3() {
        let s = "DDI";
        let arr = Solution::di_string_match(s.into());
        assert!(is_valid(s, &arr));
    }
}
