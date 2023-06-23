pub struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|s| {
                if s.chars().all(|c| c.is_ascii_digit()) {
                    s.parse().unwrap()
                } else {
                    s.len()
                }
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = ["alic3", "bob", "3", "4", "00000"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(5, Solution::maximum_value(strs));
    }

    #[test]
    fn case2() {
        let strs = ["1", "01", "001", "0001"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(1, Solution::maximum_value(strs));
    }
}
