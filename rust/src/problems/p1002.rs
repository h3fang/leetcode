pub struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        fn signature(s: &str) -> [usize; 26] {
            let mut r = [0; 26];
            for b in s.as_bytes() {
                r[(b - b'a') as usize] += 1;
            }
            r
        }

        let mut sig = [usize::MAX; 26];

        words.iter().for_each(|s| {
            let c = signature(s);
            for (i, n) in c.iter().enumerate() {
                sig[i] = sig[i].min(*n);
            }
        });
        (0..26)
            .flat_map(|i| {
                let c = (i + b'a') as char;
                let n = sig[i as usize];
                vec![c.to_string(); n]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["bella", "label", "roller"];
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let expected = ["e", "l", "l"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::common_chars(words));
    }

    #[test]
    fn case2() {
        let words = ["cool", "lock", "cook"];
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let expected = ["c", "o"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::common_chars(words));
    }
}
