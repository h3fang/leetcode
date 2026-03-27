pub struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat[0].len();
        if k % n as i32 == 0 {
            return true;
        }

        for b in mat {
            for (j, &e) in b.iter().enumerate() {
                if b[(j + k as usize) % n] != e {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::are_similar(mat, 2));
    }

    #[test]
    fn case2() {
        let mat = [[2, 2], [2, 2]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::are_similar(mat, 3));
    }

    #[test]
    fn case3() {
        let mat = [[1, 2]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::are_similar(mat, 1));
    }
}
