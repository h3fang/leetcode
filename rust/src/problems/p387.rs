pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut f = [[0, -1]; 26];
        for (i, &b) in s.as_bytes().iter().enumerate() {
            let j = (b - b'a') as usize;
            f[j][0] += 1;
            if f[j][1] == -1 {
                f[j][1] = i as i32;
            }
        }
        f.sort_unstable_by_key(|e| e[1]);
        for e in f {
            if e[0] == 1 {
                return e[1];
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
        assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::first_uniq_char("aabb".to_string()));
    }
}
