pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        fn parse(ver: &str) -> Vec<i32> {
            ver.split('.').map(|v| v.parse().unwrap()).collect()
        }
        let mut v1 = parse(&version1);
        let mut v2 = parse(&version2);
        match v1.len().cmp(&v2.len()) {
            std::cmp::Ordering::Less => v1.resize(v2.len(), 0),
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => v2.resize(v1.len(), 0),
        }
        match v1.cmp(&v2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            0,
            Solution::compare_version("1.01".to_string(), "1.001".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::compare_version("0.1".to_string(), "1.1".to_string())
        );
    }
}
