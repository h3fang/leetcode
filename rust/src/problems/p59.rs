pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut len = n;
        let mut k = 1;
        let mut i = 0;
        let mut j = -1;
        let mut d = 0;
        let mut num = 1;
        let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result = vec![vec![0; n as usize]; n as usize];
        while len > 0 {
            for _ in 0..len {
                i += dir[d].0;
                j += dir[d].1;
                result[i as usize][j as usize] = num;
                num += 1;
            }
            k += 1;
            if k == 2 {
                k = 0;
                len -= 1;
            }
            d = (d + 1) % 4;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::generate_matrix(3));
    }

    #[test]
    fn case2() {
        let expected = [[1]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::generate_matrix(1));
    }
}
