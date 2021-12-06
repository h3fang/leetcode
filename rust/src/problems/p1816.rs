pub struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.splitn(k as usize + 1, ' ')
            .take(k as usize)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "Hello how are you Contestant".to_string();
        let k = 4;
        assert_eq!(
            "Hello how are you".to_string(),
            Solution::truncate_sentence(s, k)
        );
    }
}
