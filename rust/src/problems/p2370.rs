pub struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut max = [0; 26];
        max[(s[0] - b'a') as usize] = 1;
        for &b in s.iter().skip(1) {
            let j = (b - b'a') as usize;
            let mut m = 1;
            for prev in b'a'..=b'z' {
                let diff = (b as i32 - prev as i32).abs();
                if diff > k {
                    continue;
                }
                m = m.max(max[(prev - b'a') as usize] + 1);
            }
            max[j] = m;
        }
        *max.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::longest_ideal_string("acfgbd".into(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::longest_ideal_string("abcd".into(), 3));
    }
}
