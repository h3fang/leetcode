pub struct Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut r = sentence
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, w)| {
                let s = if [b'a', b'e', b'i', b'o', b'u']
                    .contains(&w.as_bytes()[0].to_ascii_lowercase())
                {
                    w.to_string()
                } else {
                    w[1..].to_string() + &w[..1]
                };
                s + "ma" + &"a".repeat(i + 1) + " "
            })
            .collect::<String>();
        r.pop();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa",
            Solution::to_goat_latin("I speak Goat Latin".into())
        );
    }
}
