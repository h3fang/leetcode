pub struct Solution;

impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = [-1; 1 << 5];
        for (i, r) in grid.iter().enumerate() {
            let v = r.iter().fold(0, |acc, v| (acc << 1) + v);
            if v == 0 {
                return vec![i as i32];
            }
            m[v as usize] = i as i32;
        }
        for (i, &a) in m.iter().enumerate() {
            if a == -1 {
                continue;
            }
            for (j, &b) in m.iter().enumerate().skip(i + 1) {
                if b == -1 {
                    continue;
                }
                if i & j == 0 {
                    return vec![a.min(b), a.max(b)];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 1, 0], [0, 0, 0, 1], [1, 1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![0, 1], Solution::good_subsetof_binary_matrix(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![0], Solution::good_subsetof_binary_matrix(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 1, 1], [1, 1, 1]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::good_subsetof_binary_matrix(grid).is_empty());
    }
}
