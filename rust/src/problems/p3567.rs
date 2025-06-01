pub struct Solution;

use std::collections::BTreeMap;

fn min_diff(map: &BTreeMap<i32, u32>) -> i32 {
    let mut prev = i32::MIN / 2;
    let mut ans = i32::MAX / 2;
    for &x in map.keys() {
        ans = ans.min(x - prev);
        prev = x;
    }
    if ans == i32::MAX / 2 { 0 } else { ans }
}

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
        let mut ans = vec![vec![0; n - k + 1]; m - k + 1];
        if k == 1 {
            return ans;
        }
        let mut map = BTreeMap::new();
        for (i, r) in ans.iter_mut().enumerate() {
            map.clear();
            for g in &grid[i..i + k] {
                for &c in &g[..k] {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
            r[0] = min_diff(&map);
            for (j, c) in r.iter_mut().enumerate().skip(1) {
                for row in &grid[i..i + k] {
                    *map.entry(row[j + k - 1]).or_insert(0) += 1;
                    let v = row[j - 1];
                    let e = map.get_mut(&v).unwrap();
                    *e -= 1;
                    if *e == 0 {
                        map.remove(&v);
                    }
                }
                *c = min_diff(&map);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 8], [3, -2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![vec![2]], Solution::min_abs_diff(grid, 2));
    }

    #[test]
    fn case2() {
        let grid = [[3, -1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![vec![0, 0]], Solution::min_abs_diff(grid, 1));
    }

    #[test]
    fn case3() {
        let grid = [[1, -2, 3], [2, 3, 5]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![vec![1, 2]], Solution::min_abs_diff(grid, 2));
    }
}
