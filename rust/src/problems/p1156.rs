pub struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let t = text.as_bytes();
        let n = t.len();
        let mut count = [0; 26];
        for &b in t {
            count[(b - b'a') as usize] += 1;
        }
        let mut result = 0;
        let mut i = 0;
        while i < n {
            let a = t[i];
            let mut j = i;
            while j < n && t[j] == a {
                j += 1;
            }
            let c = j - i;
            if c < count[(a - b'a') as usize] && (j < n || i > 0) {
                result = result.max(c + 1);
            }
            let mut k = j + 1;
            while k < n && t[k] == a {
                k += 1;
            }
            result = result.max((k - i).min(count[(a - b'a') as usize]));
            i = j;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_rep_opt1("ababa".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::max_rep_opt1("aaabaaa".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::max_rep_opt1("aaabbaaa".to_string()));
    }
}
