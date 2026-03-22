pub struct Solution;

fn rotate_90cw(mat: &mut [Vec<i32>]) {
    let n = mat.len();
    for i in 0..n {
        for j in i + 1..n {
            if let Ok([a, b]) = mat.get_disjoint_mut([i, j]) {
                (a[j], b[i]) = (b[i], a[j]);
            }
        }
        mat[i].reverse();
    }
}

impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        for i in 0..4 {
            if mat == target {
                return true;
            }
            if i < 3 {
                rotate_90cw(&mut mat);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[0, 1], [1, 0]].iter().map(|r| r.to_vec()).collect();
        let target = [[1, 0], [0, 1]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::find_rotation(mat, target));
    }

    #[test]
    fn case2() {
        let mat = [[0, 1], [1, 1]].iter().map(|r| r.to_vec()).collect();
        let target = [[1, 0], [0, 1]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::find_rotation(mat, target));
    }

    #[test]
    fn case3() {
        let mat = [[0, 0, 0], [0, 1, 0], [1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let target = [[1, 1, 1], [0, 1, 0], [0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::find_rotation(mat, target));
    }
}
