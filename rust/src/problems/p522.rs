pub struct Solution;

impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        fn is_sub_str(s: &[u8], t: &[u8]) -> bool {
            if s.len() > t.len() {
                return false;
            }
            let mut j = 0;
            for &a in s {
                if j == t.len() {
                    return false;
                }
                while t[j] != a {
                    j += 1;
                    if j == t.len() {
                        return false;
                    }
                }
                j += 1;
            }
            true
        }

        strs.sort_unstable_by_key(|b| std::cmp::Reverse(b.len()));
        for (i, s) in strs.iter().enumerate() {
            let s = s.as_bytes();
            if strs
                .iter()
                .enumerate()
                .all(|(j, t)| i == j || !is_sub_str(s, t.as_bytes()))
            {
                return s.len() as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = ["aba", "cdc", "eae"];
        let strs = strs.iter().map(|s| s.to_string()).collect();
        assert_eq!(3, Solution::find_lu_slength(strs));
    }

    #[test]
    fn case2() {
        let strs = ["aaa", "aaa", "aa"];
        let strs = strs.iter().map(|s| s.to_string()).collect();
        assert_eq!(-1, Solution::find_lu_slength(strs));
    }
}
