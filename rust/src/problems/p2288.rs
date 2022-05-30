pub struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let mut tokens = sentence
            .split_ascii_whitespace()
            .map(|t| t.to_string())
            .collect::<Vec<_>>();
        let discount = (100 - discount) as f64 / 100.0;
        for t in tokens.iter_mut() {
            if t.as_bytes()[0] == b'$' {
                if let Ok(n) = t[1..].parse::<i64>() {
                    if n >= 0 {
                        *t = format!("${:.2}", n as f64 * discount);
                    }
                }
            }
        }
        tokens.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "there are $0.50 $1.00 and 5$ candies in the shop",
            Solution::discount_prices("there are $1 $2 and 5$ candies in the shop".into(), 50)
        );
    }
}
