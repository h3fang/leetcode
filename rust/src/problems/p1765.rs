use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let dir = [-1, 0, 1, 0, -1];
        let (n, m) = (is_water.len(), is_water[0].len());
        let mut fifo = VecDeque::with_capacity(n * m);
        is_water.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, t)| {
                if *t == 1 {
                    fifo.push_back((i as i32, j as i32))
                }
                *t -= 1;
            })
        });
        while let Some((i, j)) = fifo.pop_front() {
            let cur_high = is_water[i as usize][j as usize];
            for d in 0..4 {
                let (x, y) = (i + dir[d], j + dir[d + 1]);
                if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 {
                    continue;
                }
                if is_water[x as usize][y as usize] != -1 {
                    continue;
                }
                is_water[x as usize][y as usize] = cur_high + 1;
                fifo.push_back((x, y));
            }
        }
        is_water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let is_water = [[0, 1], [0, 0]];
        let is_water = is_water.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let expected = [[1, 0], [2, 1]];
        let expected = expected.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::highest_peak(is_water));
    }

    #[test]
    fn case2() {
        let is_water = [[0, 0, 1], [1, 0, 0], [0, 0, 0]];
        let is_water = is_water.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let expected = [[1, 1, 0], [0, 1, 1], [1, 2, 2]];
        let expected = expected.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::highest_peak(is_water));
    }
}
