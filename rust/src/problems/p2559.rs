pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn is_vowel(b: u8) -> bool {
            matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
        }
        let mut ps = vec![0; words.len() + 1];
        ps[0] = 0;
        for (i, w) in words.into_iter().enumerate() {
            let w = w.as_bytes();
            ps[i + 1] = ps[i] + i32::from(is_vowel(w[0]) && is_vowel(*w.last().unwrap()));
        }
        queries
            .into_iter()
            .map(|q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                ps[r + 1] - ps[l]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["aba", "bcb", "ece", "aa", "e"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let queries = [[0, 2], [1, 4], [1, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![2, 3, 0], Solution::vowel_strings(words, queries));
    }

    #[test]
    fn case2() {
        let words = ["a", "e", "i"].iter().map(|w| w.to_string()).collect();
        let queries = [[0, 2], [0, 1], [2, 2]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![3, 2, 1], Solution::vowel_strings(words, queries));
    }
}
