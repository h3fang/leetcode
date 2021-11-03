use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut queue = BinaryHeap::new();
        for (i, row) in height_map.iter_mut().enumerate() {
            queue.push((-row[0], i, 0));
            row[0] = i32::MAX;
            queue.push((-row[n - 1], i, n - 1));
            row[n - 1] = i32::MAX;
        }
        for j in 1..n - 1 {
            queue.push((-height_map[0][j], 0, j));
            height_map[0][j] = i32::MAX;
            queue.push((-height_map[m - 1][j], m - 1, j));
            height_map[m - 1][j] = i32::MAX;
        }

        let mut result = 0;
        const DXY: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some((h, x, y)) = queue.pop() {
            for (dx, dy) in DXY {
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if height_map[x][y] < i32::MAX {
                    if height_map[x][y] >= -h {
                        queue.push((-height_map[x][y], x, y));
                    } else {
                        result += -h - height_map[x][y];
                        queue.push((h, x, y));
                    }
                    height_map[x][y] = i32::MAX;
                }
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
        let height_map = [[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]];
        let height_map = height_map.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(4, Solution::trap_rain_water(height_map));
    }

    #[test]
    fn case2() {
        let height_map = [
            [3, 3, 3, 3, 3],
            [3, 2, 2, 2, 3],
            [3, 2, 1, 2, 3],
            [3, 2, 2, 2, 3],
            [3, 3, 3, 3, 3],
        ];
        let height_map = height_map.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(10, Solution::trap_rain_water(height_map));
    }
}
