pub struct Solution;

const VALID: [i8; 10] = [0, 0, 1, -1, -1, 1, 1, -1, 0, 1];

impl Solution {
    pub fn rotated_digits(mut n: i32) -> i32 {
        let mut digits = Vec::with_capacity(16);
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();

        fn dfs(
            p: usize,
            bound: bool,
            diff: bool,
            digits: &[i32],
            cache: &mut [Vec<Vec<i32>>],
        ) -> i32 {
            if p == digits.len() {
                return diff.into();
            }
            if cache[p][bound as usize][diff as usize] != -1 {
                return cache[p][bound as usize][diff as usize];
            }
            let mut result = 0;
            for i in 0..=(if bound { digits[p] } else { 9 }) {
                if VALID[i as usize] != -1 {
                    result += dfs(
                        p + 1,
                        bound && (i == digits[p]),
                        diff || VALID[i as usize] == 1,
                        digits,
                        cache,
                    )
                }
            }
            cache[p][bound as usize][diff as usize] = result;
            result
        }

        let mut cache = vec![vec![vec![-1; 2]; 2]; 5];
        dfs(0, true, false, &digits, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::rotated_digits(10));
    }
}
