pub struct Solution;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let n = (rows * cols) as usize;
        let is_valid = |r: i32, c: i32| -> bool { r >= 0 && c >= 0 && r < rows && c < cols };
        let mut result = Vec::with_capacity(n);
        let (mut r, mut c, mut d, mut len, mut i) = (r_start, c_start, 0, 1, 0);
        result.push(vec![r, c]);
        while result.len() < n {
            for _ in 0..len {
                let dir = &DIRS[d as usize];
                r += dir.0;
                c += dir.1;
                if is_valid(r, c) {
                    result.push(vec![r, c]);
                }
            }
            d = (d + 1) % 4;
            i += 1;
            if i % 2 == 0 {
                len += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [[0, 0], [0, 1], [0, 2], [0, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::spiral_matrix_iii(1, 4, 0, 0));
    }

    #[test]
    fn case2() {
        let expected = [
            [1, 4],
            [1, 5],
            [2, 5],
            [2, 4],
            [2, 3],
            [1, 3],
            [0, 3],
            [0, 4],
            [0, 5],
            [3, 5],
            [3, 4],
            [3, 3],
            [3, 2],
            [2, 2],
            [1, 2],
            [0, 2],
            [4, 5],
            [4, 4],
            [4, 3],
            [4, 2],
            [4, 1],
            [3, 1],
            [2, 1],
            [1, 1],
            [0, 1],
            [4, 0],
            [3, 0],
            [2, 0],
            [1, 0],
            [0, 0],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(expected, Solution::spiral_matrix_iii(5, 6, 1, 4));
    }
}
