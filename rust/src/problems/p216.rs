pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn bt(k: i32, n: i32, i: i32, used: u16, result: &mut Vec<Vec<i32>>) {
            if n == 0 && used.count_ones() == k as u32 {
                let mut comb = Vec::with_capacity(k as usize);
                for i in 1..=9 {
                    if used & (1 << i) > 0 {
                        comb.push(i);
                    }
                }
                result.push(comb);
                return;
            }
            if n < 0 || i > 9 || n > (9 + i) * (9 - i + 1) / 2 {
                return;
            }
            bt(k, n, i + 1, used, result);
            bt(k, n - i, i + 1, used | (1 << i), result);
        }
        let mut result = vec![];
        bt(k, n, 1, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(mut a: Vec<Vec<i32>>, mut b: Vec<Vec<i32>>) {
        for arr in [&mut a, &mut b] {
            arr.iter_mut().for_each(|c| c.sort_unstable());
            arr.sort_unstable();
        }
        assert_eq!(a, b);
    }

    #[test]
    fn case1() {
        let expected = [[1, 2, 4]];
        let expected = expected.iter().map(|arr| arr.to_vec()).collect::<Vec<_>>();
        assert_valid(expected, Solution::combination_sum3(3, 7));
    }

    #[test]
    fn case2() {
        let expected = [[1, 2, 6], [1, 3, 5], [2, 3, 4]];
        let expected = expected.iter().map(|arr| arr.to_vec()).collect::<Vec<_>>();
        assert_valid(expected, Solution::combination_sum3(3, 9));
    }

    #[test]
    fn case3() {
        let expected: Vec<Vec<i32>> = vec![];
        assert_valid(expected, Solution::combination_sum3(4, 1));
    }

    #[test]
    fn case4() {
        let expected: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]];
        assert_valid(expected, Solution::combination_sum3(9, 45));
    }
}
