pub struct Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::find_lu_slength("aba".to_string(), "cdc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::find_lu_slength("aba".to_string(), "aba".to_string())
        );
    }
}
