pub struct Solution;

impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(land: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
            if i >= land.len() || j >= land[0].len() || land[i][j] != 0 {
                return 0;
            }
            land[i][j] = -1;
            let mut result = 1;
            for (di, dj) in [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ] {
                result += dfs(land, (i as i64 + di) as usize, (j as i64 + dj) as usize);
            }
            result
        }
        let mut result = vec![];
        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 0 {
                    result.push(dfs(&mut land, i, j));
                }
            }
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let land = [[0, 2, 1, 0], [0, 1, 0, 1], [1, 1, 0, 1], [0, 1, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![1, 2, 4], Solution::pond_sizes(land));
    }
}
