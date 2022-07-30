pub struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut f2 = [0; 26];
        for w in &words2 {
            let mut f = [0; 26];
            for b in w.as_bytes() {
                let i = (b - b'a') as usize;
                f[i] += 1;
            }
            f2.iter_mut().zip(f).for_each(|(a, b)| *a = (*a).max(b));
        }

        let mut result = vec![];
        for w in words1 {
            let mut f = [0; 26];
            for b in w.as_bytes() {
                let i = (b - b'a') as usize;
                f[i] += 1;
            }
            if f.iter().zip(&f2).all(|(a, b)| a >= b) {
                result.push(w);
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
        let words1 = ["amazon", "apple", "facebook", "google", "leetcode"];
        let words2 = ["e", "o"];
        let words1 = words1.iter().map(|w| w.to_string()).collect();
        let words2 = words2.iter().map(|w| w.to_string()).collect();
        let mut result = Solution::word_subsets(words1, words2);
        result.sort_unstable();
        let expected = ["facebook", "google", "leetcode"];
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
