pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        mat.iter()
            .enumerate()
            .map(|(i, r)| r[i] + if n - 1 - i != i { r[n - 1 - i] } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(25, Solution::diagonal_sum(mat));
    }

    #[test]
    fn case2() {
        let mat = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(8, Solution::diagonal_sum(mat));
    }

    #[test]
    fn case3() {
        let mat = [[5]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(5, Solution::diagonal_sum(mat));
    }
}
