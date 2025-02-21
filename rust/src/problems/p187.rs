pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        fn binary(b: u8) -> u32 {
            match b {
                b'A' => 0,
                b'C' => 1,
                b'G' => 2,
                _ => 3,
            }
        }
        let bytes = s.as_bytes();
        let mut result = vec![];
        if bytes.len() <= 10 {
            return result;
        }
        let mut m = HashMap::new();
        let mut x = 0;
        for &b in &bytes[..9] {
            x = (x << 2) | binary(b);
        }
        for i in 0..=bytes.len() - 10 {
            x = ((x << 2) | binary(bytes[i + 10 - 1])) & ((1 << 20) - 1);
            let e = m.entry(x).or_insert(0);
            *e += 1;
            if *e == 2 {
                result.push(s[i..i + 10].to_string());
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
        let mut result =
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into());
        result.sort_unstable();
        let expected = ["AAAAACCCCC", "CCCCCAAAAA"];
        let mut expected = expected.iter().map(|p| p.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
