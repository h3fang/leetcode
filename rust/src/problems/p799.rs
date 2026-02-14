pub struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut f = [0.0; 101];
        f[0] = poured as f64;
        for r in 1..=query_row as usize {
            for c in (0..=r).rev() {
                let overflow = (f[c] - 1.0) / 2.0;
                if overflow > 0.0 {
                    f[c] = overflow;
                    f[c + 1] += overflow;
                } else {
                    f[c] = 0.0;
                }
            }
        }
        f[query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "a = {a:.9}, b = {b:.9}");
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
