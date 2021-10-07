pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let expected = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn case2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let expected = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        Solution::reverse_string(&mut s);
        assert_eq!(expected, s);
    }
}
