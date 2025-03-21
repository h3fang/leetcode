pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut deg = 1;
        for p in preorder.split(',') {
            deg -= 1;
            if deg < 0 {
                return false;
            }
            if p != "#" {
                deg += 2;
            }
        }
        deg == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_valid_serialization(
            "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_valid_serialization("1,#".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_valid_serialization("9,#,#,1".to_string()));
    }
}
