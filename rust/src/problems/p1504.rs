pub struct Solution;

impl Solution {
    pub fn num_submat(mut mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..mat.len() {
            let mut s = vec![(-1, 0, -1)];
            for j in 0..mat[i].len() {
                if i > 0 && mat[i][j] == 1 {
                    mat[i][j] = mat[i - 1][j] + 1;
                }
                let h = mat[i][j];
                while s.last().unwrap().2 >= h {
                    s.pop();
                }
                let last = s.last().unwrap();
                let cnt = last.1 + (j as i32 - last.0) * h;
                ans += cnt;
                s.push((j as i32, cnt, h));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(13, Solution::num_submat(mat));
    }

    #[test]
    fn case2() {
        let mat = [[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(24, Solution::num_submat(mat));
    }
}
