pub struct Solution;

impl Solution {
    pub fn is_fliped_string(mut s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        s1 += &s1.to_string();
        s1.contains(&s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::is_fliped_string("waterbottle".to_string(), "erbottlewat".to_string())
        );
    }
}
