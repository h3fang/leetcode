#[allow(dead_code)]
pub struct Solution {
    bad: i32,
}

impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, versions: i32) -> bool {
        versions >= self.bad
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut a = 1;
        let mut b = n;
        let mut result = n;
        while a <= b {
            let v = a + (b - a) / 2;
            if self.isBadVersion(v) {
                result = v;
                b = v - 1;
            } else {
                a = v + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bad = 4;
        let s = Solution { bad };
        assert_eq!(bad, s.first_bad_version(5));
    }

    #[test]
    fn case2() {
        let bad = 1;
        let s = Solution { bad };
        assert_eq!(bad, s.first_bad_version(1));
    }

    #[test]
    fn case3() {
        let bad = 1702766719;
        let s = Solution { bad };
        assert_eq!(bad, s.first_bad_version(2126753390));
    }
}
