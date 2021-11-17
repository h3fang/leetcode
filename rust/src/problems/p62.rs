pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut result = 1i64;
        let mut x = n;
        let mut y = 1;
        while y < m {
            result = result * x as i64 / y as i64;
            y += 1;
            x += 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(28, Solution::unique_paths(3, 7));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::unique_paths(3, 2));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::unique_paths(3, 3));
    }
}
