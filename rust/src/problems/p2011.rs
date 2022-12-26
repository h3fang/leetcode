pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let p = operations
            .iter()
            .filter(|&op| op == "X++" || op == "++X")
            .count() as i32;
        2 * p - operations.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let operations = ["--X", "X++", "X++"]
            .iter()
            .map(|op| op.to_string())
            .collect();
        assert_eq!(1, Solution::final_value_after_operations(operations));
    }
}
