pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (mut first_min_sum, mut sec_min_sum, mut first_min_index) = (0, 0, usize::MAX);
        for row in &grid {
            let (mut cur_first_min_sum, mut cur_sec_min_sum, mut cur_first_min_index) =
                (i32::MAX, i32::MAX, usize::MAX);
            for (j, c) in row.iter().enumerate() {
                let cur_sum = if j == first_min_index {
                    sec_min_sum
                } else {
                    first_min_sum
                } + c;
                if cur_sum < cur_first_min_sum {
                    cur_sec_min_sum = cur_first_min_sum;
                    cur_first_min_sum = cur_sum;
                    cur_first_min_index = j;
                } else if cur_sum < cur_sec_min_sum {
                    cur_sec_min_sum = cur_sum;
                }
            }
            first_min_sum = cur_first_min_sum;
            sec_min_sum = cur_sec_min_sum;
            first_min_index = cur_first_min_index;
        }
        first_min_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(13, Solution::min_falling_path_sum(grid));
    }

    #[test]
    fn case2() {
        let grid = [[7]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(7, Solution::min_falling_path_sum(grid));
    }
}
