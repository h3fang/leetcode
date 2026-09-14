pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut map: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        for (i, r) in mat.iter().enumerate() {
            for (j, &v) in r.iter().enumerate() {
                map.entry(v).or_default().push((i, j));
            }
        }
        let (mut row, mut col) = (vec![0; m], vec![0; n]);
        for pos in map.values() {
            let max = pos
                .iter()
                .map(|&(i, j)| row[i].max(col[j]) + 1)
                .collect::<Vec<_>>();
            for (&(i, j), m) in pos.iter().zip(max) {
                row[i] = row[i].max(m);
                col[j] = col[j].max(m);
            }
        }
        row.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[3, 1], [3, 4]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::max_increasing_cells(mat));
    }

    #[test]
    fn case2() {
        let mat = [[1, 1], [1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::max_increasing_cells(mat));
    }

    #[test]
    fn case3() {
        let mat = [[3, 1, 6], [-9, 5, 7]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::max_increasing_cells(mat));
    }
}
