pub struct Solution;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort_unstable();
        let mut b = bottom;
        let mut result = 0;
        for s in special {
            if s > b {
                result = result.max(s - b);
            }
            b = s + 1;
        }
        result = result.max(top - b + 1);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_consecutive(2, 9, vec![4, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_consecutive(6, 8, vec![7, 6, 8]));
    }
}
