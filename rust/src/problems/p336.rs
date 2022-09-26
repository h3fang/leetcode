pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let word_map: HashMap<_, _> = words
            .iter()
            .enumerate()
            .map(|(i, w)| (w.as_str(), i))
            .collect();

        words.iter().enumerate().fold(vec![], |mut acc, (i, w)| {
            let n = w.len();
            let rev: String = w.chars().rev().collect();
            let from_right = (0..n)
                .filter(|&k| w[..n - k] == rev[k..])
                .filter_map(|k| word_map.get(&rev[..k]).map(|&j| (j, i)));

            (0..=n)
                .filter(|&k| w[k..] == rev[..n - k])
                .filter_map(|k| word_map.get(&rev[n - k..]).map(|&j| (i, j)))
                .chain(from_right)
                .filter(|&(i, j)| i != j)
                .for_each(|(i, j)| acc.push(vec![i as i32, j as i32]));

            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abcd", "dcba", "lls", "s", "sssll"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let mut expected = [[0, 1], [1, 0], [3, 2], [2, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        let mut result = Solution::palindrome_pairs(words);
        expected.sort_unstable();
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
