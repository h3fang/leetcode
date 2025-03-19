pub struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut freq_s = [0; 10];
        let mut freq_g = [0; 10];
        let a = secret
            .as_bytes()
            .iter()
            .zip(guess.as_bytes())
            .filter(|&(&c1, &c2)| {
                freq_s[(c1 - b'0') as usize] += 1;
                freq_g[(c2 - b'0') as usize] += 1;
                c1 == c2
            })
            .count();

        let b: usize = freq_s.iter().zip(&freq_g).map(|(f1, f2)| f1.min(f2)).sum();

        format!("{}A{}B", a, b - a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let secret = "1807".to_string();
        let guess = "7810".to_string();
        assert_eq!("1A3B".to_string(), Solution::get_hint(secret, guess));
    }

    #[test]
    fn case2() {
        let secret = "1123".to_string();
        let guess = "0111".to_string();
        assert_eq!("1A1B".to_string(), Solution::get_hint(secret, guess));
    }

    #[test]
    fn case3() {
        let secret = "0".to_string();
        let guess = "1".to_string();
        assert_eq!("0A0B".to_string(), Solution::get_hint(secret, guess));
    }
}
