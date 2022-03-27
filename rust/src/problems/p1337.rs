pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut m = mat
            .iter()
            .enumerate()
            .map(|(i, row)| (row.iter().filter(|c| **c == 1).count(), i))
            .collect::<Vec<_>>();
        m.sort_unstable();
        m.iter().map(|e| e.1 as i32).take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        let k = 3;
        assert_eq!(vec![2, 0, 3], Solution::k_weakest_rows(mat, k));
    }

    #[test]
    fn case2() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let k = 2;
        assert_eq!(vec![0, 2], Solution::k_weakest_rows(mat, k));
    }
}
