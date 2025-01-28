pub struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if (m * n) as usize == original.len() {
            original.chunks(n as usize).map(|r| r.to_vec()).collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [[1, 2], [3, 4]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2)
        );
    }

    #[test]
    fn case2() {
        assert!(Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 1).is_empty());
    }
}
