pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut ys = HashSet::new();
        for r in &rectangles {
            ys.insert(r[1]);
            ys.insert(r[3]);
        }
        let mut ys = ys.into_iter().collect::<Vec<_>>();
        ys.sort_unstable();
        let m = ys.len();
        let mut seg = vec![0; m - 1];
        let mut sweep_lines = vec![];
        for (i, r) in rectangles.iter().enumerate() {
            sweep_lines.push((r[0], i, 1));
            sweep_lines.push((r[2], i, -1));
        }
        sweep_lines.sort_unstable();
        let mut result = 0;
        let mut i = 0;
        while i < sweep_lines.len() {
            let mut j = i;
            while j + 1 < sweep_lines.len() && sweep_lines[j + 1].0 == sweep_lines[i].0 {
                j += 1;
            }
            if j + 1 == sweep_lines.len() {
                break;
            }
            for &(_, idx, diff) in &sweep_lines[i..=j] {
                let bottom = rectangles[idx][1];
                let top = rectangles[idx][3];
                for x in 0..m - 1 {
                    if bottom <= ys[x] && ys[x + 1] <= top {
                        seg[x] += diff;
                    }
                }
            }
            let h: i64 = seg
                .iter()
                .enumerate()
                .map(|(k, &s)| (s.max(0).min(1) * (ys[k + 1] - ys[k])) as i64)
                .sum();
            result += h * (sweep_lines[j + 1].0 - sweep_lines[j].0) as i64;
            i = j + 1;
        }
        (result % 10_0000_0007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rectangles = [[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(6, Solution::rectangle_area(rectangles));
    }

    #[test]
    fn case2() {
        let rectangles = [[0, 0, 1000000000, 1000000000]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(49, Solution::rectangle_area(rectangles));
    }
}
