pub struct Solution;

const CAT: [&str; 4] = ["electronics", "grocery", "pharmacy", "restaurant"];

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut valid = Vec::with_capacity(code.len());
        for (i, ((code, bl), &active)) in
            code.iter().zip(&business_line).zip(&is_active).enumerate()
        {
            if !code.is_empty()
                && code.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                && CAT.contains(&bl.as_str())
                && active
            {
                valid.push(i);
            }
        }
        valid.sort_unstable_by_key(|&i| (&business_line[i], &code[i]));
        valid.into_iter().map(|i| code[i].to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = ["PHARMA5", "SAVE20"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let code = ["SAVE20", "", "PHARMA5", "SAVE@20"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let business_line = ["restaurant", "grocery", "pharmacy", "restaurant"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let is_active = vec![true, true, true, true];
        assert_eq!(
            expected,
            Solution::validate_coupons(code, business_line, is_active)
        );
    }

    #[test]
    fn case2() {
        let expected = ["ELECTRONICS_50"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let code = ["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let business_line = ["grocery", "electronics", "invalid"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let is_active = vec![false, true, true];
        assert_eq!(
            expected,
            Solution::validate_coupons(code, business_line, is_active)
        );
    }
}
