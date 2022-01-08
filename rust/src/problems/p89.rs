pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|i| (i >> 1) ^ i).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![0, 1], Solution::gray_code(1));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8],
            Solution::gray_code(4)
        );
    }
}
