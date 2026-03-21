pub struct Solution;

impl Solution {
    pub fn num_special(mut mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let sum = mat[0].iter().sum::<i32>() - 1;
        mat[0]
            .iter_mut()
            .filter(|x| **x == 1)
            .for_each(|x| *x += sum);
        for i in 1..m {
            let sum = mat[i].iter().sum::<i32>() - i32::from(i == 0);
            if sum > 0
                && let Ok([a, b]) = mat.get_disjoint_mut([0, i])
            {
                for j in 0..n {
                    if b[j] == 1 {
                        a[j] += sum;
                    }
                }
            }
        }
        mat[0].iter().filter(|&&e| e == 1).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 0, 0], [0, 0, 1], [1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::num_special(mat));
    }

    #[test]
    fn case2() {
        let mat = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::num_special(mat));
    }

    #[test]
    fn case3() {
        let mat = [[0, 0, 0, 1], [1, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::num_special(mat));
    }
}
