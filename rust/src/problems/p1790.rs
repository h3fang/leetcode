pub struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut i1 = -1;
        let mut i2 = -1;
        for (i, (a, b)) in s1.iter().zip(s2).enumerate() {
            if a != b {
                if i1 == -1 {
                    i1 = i as i32;
                } else if i2 == -1 {
                    i2 = i as i32;
                    if s1[i1 as usize] != s2[i2 as usize] || s2[i1 as usize] != s1[i2 as usize] {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        !(i1 != -1 && i2 == -1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::are_almost_equal(
            "bank".to_string(),
            "kanb".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::are_almost_equal(
            "attack".to_string(),
            "defend".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(Solution::are_almost_equal(
            "bank".to_string(),
            "bank".to_string()
        ));
    }
}
