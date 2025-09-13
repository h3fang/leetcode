pub struct Solution;

const VOWELS: [usize; 5] = [
    0,
    (b'e' - b'a') as usize,
    (b'i' - b'a') as usize,
    (b'o' - b'a') as usize,
    (b'u' - b'a') as usize,
];

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut f = [0; 26];
        for b in s.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        let (mut m1, mut m2) = (0, 0);
        for (i, f) in f.into_iter().enumerate() {
            if VOWELS.contains(&i) {
                m1 = m1.max(f);
            } else {
                m2 = m2.max(f);
            }
        }
        m1 + m2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::max_freq_sum("successes".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_freq_sum("aeiaeia".to_string()));
    }
}
