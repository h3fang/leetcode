pub struct Solution;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut count = [0; 26];
        for b in license_plate.as_bytes() {
            if b.is_ascii_alphabetic() {
                count[(b.to_ascii_lowercase() - b'a') as usize] += 1;
            }
        }

        let mut result = "".to_string();
        for w in words {
            let mut c = count;
            for b in w.as_bytes() {
                c[(b - b'a') as usize] -= 1;
            }
            if c.iter().all(|&e| e <= 0) && (result.is_empty() || w.len() < result.len()) {
                result = w;
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
        let license_plate = "1s3 PSt".to_string();
        let words = ["step", "steps", "stripe", "stepple"];
        let words = words.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(
            "steps",
            Solution::shortest_completing_word(license_plate, words)
        );
    }

    #[test]
    fn case2() {
        let license_plate = "1s3 456".to_string();
        let words = ["looks", "pest", "stew", "show"];
        let words = words.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(
            "pest",
            Solution::shortest_completing_word(license_plate, words)
        );
    }

    #[test]
    fn case3() {
        let license_plate = "Ah71752".to_string();
        let words = [
            "suggest",
            "letter",
            "of",
            "husband",
            "easy",
            "education",
            "drug",
            "prevent",
            "writer",
            "old",
        ];
        let words = words.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(
            "husband",
            Solution::shortest_completing_word(license_plate, words)
        );
    }
}
