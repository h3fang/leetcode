pub struct Solution;

impl Solution {
    pub fn odd_string(mut words: Vec<String>) -> String {
        let diff = words
            .iter()
            .map(|w| {
                let w = w.as_bytes();
                w.iter()
                    .map(|&b| b as i16 - w[0] as i16)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for (i, w) in diff.windows(3).enumerate() {
            if w[0] != w[1] && w[0] != w[2] {
                return words.swap_remove(i);
            }
            if w[0] != w[1] && w[1] != w[2] {
                return words.swap_remove(i + 1);
            }
            if w[0] != w[2] && w[1] != w[2] {
                return words.swap_remove(i + 2);
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["adc", "wzy", "abc"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!("abc", Solution::odd_string(words));
    }

    #[test]
    fn case2() {
        let words = ["aaa", "bob", "ccc", "ddd"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!("bob", Solution::odd_string(words));
    }
}
