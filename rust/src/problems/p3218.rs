pub struct Solution;

impl Solution {
    pub fn minimum_cost(m: i32, n: i32, mut h_cut: Vec<i32>, mut v_cut: Vec<i32>) -> i32 {
        h_cut.sort_unstable();
        v_cut.sort_unstable();
        let (mut result, mut i, mut j) = (0, 0, 0);
        for _ in 0..(m + n - 2) {
            if j == n - 1 || (i < m - 1 && h_cut[i as usize] < v_cut[j as usize]) {
                result += h_cut[i as usize] * (n - j);
                i += 1;
            } else {
                result += v_cut[j as usize] * (m - i);
                j += 1;
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
        assert_eq!(13, Solution::minimum_cost(3, 2, vec![1, 3], vec![5]));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::minimum_cost(2, 2, vec![7], vec![4]));
    }
}
