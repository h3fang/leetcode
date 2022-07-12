pub struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];
        for i in indices {
            rows[i[0] as usize] += 1;
            cols[i[1] as usize] += 1;
        }
        let ox = rows.iter().filter(|e| *e % 2 == 1).count() as i32;
        let oy = cols.iter().filter(|e| *e % 2 == 1).count() as i32;
        (m - ox) * oy + (n - oy) * ox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let m = 2;
        let n = 3;
        let indices = [[0, 1], [1, 1]];
        let indices = indices.iter().map(|i| i.to_vec()).collect();
        assert_eq!(6, Solution::odd_cells(m, n, indices));
    }
}
