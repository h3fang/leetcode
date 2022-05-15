pub struct Solution;

impl Solution {
    pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        tiles.sort_unstable();
        let mut start_index = 0;
        let mut start_offset = 0;
        let mut covered = 0;
        let mut result = 0;
        for (i, t) in tiles.iter().enumerate() {
            covered += t[1] - t[0] + 1;
            let end = tiles[i][1];

            while end - (tiles[start_index][0] + start_offset) + 1 > carpet_len {
                let s = &tiles[start_index];
                if end - s[1] <= carpet_len {
                    let d = end - (s[0] + start_offset) + 1 - carpet_len;
                    start_offset += d;
                    covered -= d;
                } else {
                    covered -= s[1] - s[0] + 1 - start_offset;
                    start_index += 1;
                    start_offset = 0;
                }
            }
            result = result.max(covered);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tiles = [[1, 5], [10, 11], [12, 18], [20, 25], [30, 32]];
        let tiles = tiles.iter().map(|t| t.to_vec()).collect();
        let carpet_len = 10;
        assert_eq!(9, Solution::maximum_white_tiles(tiles, carpet_len));
    }

    #[test]
    fn case2() {
        let tiles = [[10, 11], [1, 1]];
        let tiles = tiles.iter().map(|t| t.to_vec()).collect();
        let carpet_len = 2;
        assert_eq!(2, Solution::maximum_white_tiles(tiles, carpet_len));
    }
}
