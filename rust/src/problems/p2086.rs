pub struct Solution;

impl Solution {
    pub fn minimum_buckets(street: String) -> i32 {
        let s = street.as_bytes();
        let mut result = 0;
        let mut last_bucket = usize::MAX;
        for i in 0..s.len() {
            if s[i] == b'H' {
                if (i == 0 || s[i - 1] == b'H') && (i == s.len() - 1 || s[i + 1] == b'H') {
                    return -1;
                }

                if i > 0 && last_bucket == i - 1 {
                    continue;
                }

                if i > 0
                    && s[i - 1] == b'.'
                    && (i == s.len() - 1 || s[i + 1] == b'H')
                    && last_bucket != i - 1
                {
                    result += 1;
                    last_bucket = i - 1;
                    continue;
                }

                if i + 1 < s.len() || s[i + 1] == b'.' {
                    result += 1;
                    last_bucket = i + 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = ".HH.H.H.H..".to_string();
        assert_eq!(3, Solution::minimum_buckets(s));
    }
}
