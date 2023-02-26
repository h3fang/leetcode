pub struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let n = words.len();
        let mut count = [0; 26];
        for c in letters {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let mut result = 0;
        for s in 1..(1 << n) {
            let mut wc = [0; 26];
            for (k, w) in words.iter().enumerate() {
                if s & (1 << k) == 0 {
                    continue;
                }
                for &c in w.as_bytes() {
                    wc[(c - b'a') as usize] += 1;
                }
            }
            let mut ok = true;
            let mut sum = 0;
            for i in 0..26 {
                sum += score[i] * wc[i];
                ok = ok && (wc[i] <= count[i]);
            }
            if ok {
                result = result.max(sum);
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
        let words = ["dog", "cat", "dad", "good"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
        let score = vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(23, Solution::max_score_words(words, letters, score));
    }
}
