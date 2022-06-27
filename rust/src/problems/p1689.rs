pub struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        (n.as_bytes().iter().max().unwrap() - b'0') as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_partitions("32".into()));
    }
}
