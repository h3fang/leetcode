pub struct Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let k = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            _ => 2,
        };
        items.iter().filter(|e| e[k] == rule_value).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let items = [
            ["phone", "blue", "pixel"],
            ["computer", "silver", "lenovo"],
            ["phone", "gold", "iphone"],
        ]
        .iter()
        .map(|e| e.iter().map(|s| s.to_string()).collect())
        .collect();
        let rule_key = "color".to_string();
        let rule_value = "silver".to_string();
        assert_eq!(1, Solution::count_matches(items, rule_key, rule_value));
    }
}
