pub struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let (s, t) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());
        let mut suf = vec![0; m + 1];
        suf[m] = n as i32;
        let mut j = n as i32 - 1;
        for (i, &b) in s.iter().enumerate().rev() {
            if j >= 0 && b == t[j as usize] {
                j -= 1;
            }
            suf[i] = j + 1;
        }

        let mut ans = Vec::with_capacity(n);
        let (mut changed, mut j) = (false, 0);

        for (i, &c) in s.iter().enumerate() {
            if c == t[j] || (!changed && suf[i + 1] <= j as i32 + 1) {
                if c != t[j] {
                    changed = true;
                }
                ans.push(i as i32);
                j += 1;
                if j == n {
                    return ans;
                }
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![0, 1, 2],
            Solution::valid_sequence("vbcca".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 2, 4],
            Solution::valid_sequence("bacdc".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            vec![0, 1],
            Solution::valid_sequence("abc".to_string(), "ab".to_string())
        );
    }
}
