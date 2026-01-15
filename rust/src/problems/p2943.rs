pub struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(_n: i32, _m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        fn max_hole(mut bars: Vec<i32>) -> i32 {
            bars.sort_unstable();

            let (mut max, mut left) = (1, -1);
            for (i, &x) in bars.iter().enumerate() {
                if i == 0 || x != bars[i - 1] + 1 {
                    left = x - 1;
                }
                max = max.max(x + 1 - left);
            }
            max
        }

        let d = max_hole(h_bars).min(max_hole(v_bars));
        d * d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let h_bars = vec![2, 3];
        let v_bars = vec![2];
        assert_eq!(4, Solution::maximize_square_hole_area(2, 1, h_bars, v_bars));
    }

    #[test]
    fn case2() {
        let h_bars = vec![2];
        let v_bars = vec![2];
        assert_eq!(4, Solution::maximize_square_hole_area(1, 1, h_bars, v_bars));
    }

    #[test]
    fn case3() {
        let h_bars = vec![2, 3];
        let v_bars = vec![2, 4];
        assert_eq!(4, Solution::maximize_square_hole_area(2, 3, h_bars, v_bars));
    }
}
