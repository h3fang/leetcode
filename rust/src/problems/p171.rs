pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .as_bytes()
            .iter()
            .fold(0, |acc, b| acc * 26 + (b - b'A' + 1) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::title_to_number("A".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(28, Solution::title_to_number("AB".to_string()));
    }
}
