pub struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut index = vec![(0, 0); m * n];
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for (i, r) in mat.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                index[c as usize - 1] = (i, j);
            }
        }
        for (k, a) in arr.into_iter().enumerate() {
            let (i, j) = index[a as usize - 1];
            rows[i] += 1;
            cols[j] += 1;
            if rows[i] == n || cols[j] == m {
                return k as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![1, 3, 4, 2];
        let mat = [[1, 4], [2, 3]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::first_complete_index(arr, mat));
    }

    #[test]
    fn case2() {
        let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat = [[3, 2, 5], [1, 4, 6], [8, 7, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::first_complete_index(arr, mat));
    }
}
