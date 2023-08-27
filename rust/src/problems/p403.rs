pub struct Solution;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut f = vec![vec![false; n]; n];
        f[0][0] = true;
        for i in 1..n {
            if stones[i] - stones[i - 1] > i as i32 {
                return false;
            }
        }
        for i in 1..n {
            for j in (0..i).rev() {
                let k = (stones[i] - stones[j]) as usize;
                if k > j + 1 {
                    break;
                }
                f[i][k] = f[j][k - 1] || f[j][k] || f[j][k + 1];
                if i == n - 1 && f[i][k] {
                    return true;
                }
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
        assert!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]));
    }
}
