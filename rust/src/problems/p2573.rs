pub struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut s = vec![b' '; n];
        let mut i = 0;
        for c in b'a'..=b'z' {
            for j in i..n {
                if lcp[i][j] > 0 {
                    s[j] = c;
                }
            }
            while i < n && s[i] != b' ' {
                i += 1;
            }
            if i == n {
                break;
            }
        }

        if i < n {
            return String::new();
        }

        for (i, &a) in s.iter().enumerate().rev() {
            for (j, &b) in s.iter().enumerate().rev() {
                let v = if a == b {
                    if i == n - 1 || j == n - 1 {
                        1
                    } else {
                        lcp[i + 1][j + 1] + 1
                    }
                } else {
                    0
                };
                if v != lcp[i][j] {
                    return String::new();
                }
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let lcp = [[4, 0, 2, 0], [0, 3, 0, 1], [2, 0, 2, 0], [0, 1, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!("abab", Solution::find_the_string(lcp));
    }

    #[test]
    fn case2() {
        let lcp = [[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!("aaaa", Solution::find_the_string(lcp));
    }

    #[test]
    fn case3() {
        let lcp = [[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!("", Solution::find_the_string(lcp));
    }
}
