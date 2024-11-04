pub struct Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let y = y / 4;
        if x.min(y) % 2 == 1 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("Alice", Solution::losing_player(2, 7));
    }

    #[test]
    fn case2() {
        assert_eq!("Bob", Solution::losing_player(4, 11));
    }
}
