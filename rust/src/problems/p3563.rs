pub struct Solution;

fn is_adj(a: u8, b: u8) -> bool {
    let diff = a.abs_diff(b);
    diff == 1 || diff == 25
}

fn valid_bracket_string(s: &[u8]) -> Vec<Vec<bool>> {
    let n = s.len();
    let mut f = vec![vec![false; n]; n];
    for i in (0..n - 1).rev() {
        f[i + 1][i] = true;
        for j in (i + 1..n).step_by(2) {
            if is_adj(s[i], s[j]) && f[i + 1][j - 1] {
                f[i][j] = true;
                continue;
            }
            for k in (i + 1..j - 1).step_by(2) {
                if f[i][k] && f[k + 1][j] {
                    f[i][j] = true;
                    break;
                }
            }
        }
    }
    f
}

impl Solution {
    pub fn lexicographically_smallest_string(s: String) -> String {
        let vbs = valid_bracket_string(s.as_bytes());
        let n = s.len();
        let mut f = vec![String::new(); n + 1];
        for i in (0..n).rev() {
            let mut r = s[i..i + 1].to_string() + &f[i + 1];
            for j in (i + 1..n).step_by(2) {
                if vbs[i][j] {
                    r = r.min(f[j + 1].to_string());
                }
            }
            f[i] = r;
        }
        let mut ans = String::new();
        std::mem::swap(&mut f[0], &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "a",
            Solution::lexicographically_smallest_string("abc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "",
            Solution::lexicographically_smallest_string("bcda".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "zdce",
            Solution::lexicographically_smallest_string("zdce".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "bababeeecabbeabbaccdfcbdcddbcdffadfdeffbeafdfbf",
            Solution::lexicographically_smallest_string(
                "badabcbabeeecaefbcbbebaadcbbaefdeccdfcbdcddbcdffbcadfdeffbeafdfbf".to_string()
            )
        );
    }
}
