pub struct Solution;

impl Solution {
    pub fn maximum_rows(mat: Vec<Vec<i32>>, cols: i32) -> i32 {
        let n = mat[0].len();
        let mut result = 0;
        for mask in 1u32..(1 << n) {
            if mask.count_ones() != cols as u32 {
                continue;
            }
            let c = mat
                .iter()
                .filter(|r| {
                    r.iter()
                        .enumerate()
                        .all(|(i, &e)| e == 0 || mask & (1 << i) > 0)
                })
                .count() as i32;
            result = result.max(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let cols = 2;
        assert_eq!(3, Solution::maximum_rows(mat, cols));
    }
}
