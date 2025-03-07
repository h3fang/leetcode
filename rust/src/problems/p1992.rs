pub struct Solution;

impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (land.len(), land[0].len());
        let mut result = vec![];
        for i in 0..m {
            for j in 0..n {
                if land[i][j] != 1 {
                    continue;
                }
                let mut w = 1;
                while j + w < n && land[i][j + w] == 1 {
                    w += 1;
                }
                let mut h = 1;
                while i + h < m && land[i + h][j] == 1 {
                    h += 1;
                }
                result.push(vec![
                    i as i32,
                    j as i32,
                    (i + h - 1) as i32,
                    (j + w - 1) as i32,
                ]);
                for r in &mut land[i..i + h] {
                    for c in &mut r[j..j + w] {
                        *c = 0;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let land = [[1, 0, 0], [0, 1, 1], [0, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[0, 0, 0, 0], [1, 1, 2, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_farmland(land));
    }

    #[test]
    fn case2() {
        let land = [[1, 1], [1, 1]].iter().map(|r| r.to_vec()).collect();
        let expected = [[0, 0, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_farmland(land));
    }

    #[test]
    fn case3() {
        let land = [[0]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::find_farmland(land).is_empty());
    }
}
