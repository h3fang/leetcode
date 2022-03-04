pub struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut amount = vec![vec![0.0; 101]; 101];
        amount[0][0] = poured as f64;
        for r in 0..=query_row as usize {
            for c in 0..=r {
                let overflow = (amount[r][c] - 1.0) / 2.0;
                if overflow > 0.0 {
                    amount[r + 1][c] += overflow;
                    amount[r + 1][c + 1] += overflow;
                }
            }
        }
        amount[query_row as usize][query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9);
    }

    #[test]
    fn case1() {
        assert_close(0.0, Solution::champagne_tower(1, 1, 1));
    }

    #[test]
    fn case2() {
        assert_close(0.5, Solution::champagne_tower(2, 1, 1));
    }

    #[test]
    fn case3() {
        assert_close(1.0, Solution::champagne_tower(100000009, 33, 17));
    }
}
