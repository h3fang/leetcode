pub struct Solution;

use std::collections::HashSet;

fn lengths(n: i32, mut fences: Vec<i32>) -> HashSet<i32> {
    fences.extend_from_slice(&[1, n]);
    fences.sort_unstable();

    let mut s = HashSet::with_capacity(fences.len() * fences.len() / 2);

    for (i, &x) in fences.iter().enumerate() {
        for &y in &fences[i + 1..] {
            s.insert(y - x);
        }
    }

    s
}

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let h = lengths(m, h_fences);
        let v = lengths(n, v_fences);

        let mut max = 0;
        for x in h {
            if v.contains(&x) {
                max = max.max(x);
            }
        }

        if max == 0 {
            -1
        } else {
            ((max as i64 * max as i64) % 10_0000_0007) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let h_bars = vec![2, 3];
        let v_bars = vec![2];
        assert_eq!(4, Solution::maximize_square_area(4, 3, h_bars, v_bars));
    }

    #[test]
    fn case2() {
        let h_bars = vec![2];
        let v_bars = vec![4];
        assert_eq!(-1, Solution::maximize_square_area(6, 7, h_bars, v_bars));
    }
}
