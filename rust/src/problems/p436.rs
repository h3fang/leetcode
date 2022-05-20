pub struct Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut intervals = intervals
            .into_iter()
            .enumerate()
            .map(|(i, int)| [int[0], int[1], i as i32])
            .collect::<Vec<_>>();
        intervals.sort_unstable();
        let mut result = vec![-1; intervals.len()];
        for i in (0..intervals.len()).rev() {
            let b = intervals[i][1];
            let mut left = i as i32 + 1;
            let mut right = intervals.len() as i32 - 1;
            let mut idx = -1;
            while left <= right {
                let mid = (left + right) / 2;
                match intervals[mid as usize][0].cmp(&b) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    _ => {
                        idx = intervals[mid as usize][2];
                        right = mid - 1;
                    }
                }
            }
            result[intervals[i][2] as usize] = idx;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_intervals(ints: &[[i32; 2]]) -> Vec<Vec<i32>> {
        ints.iter().map(|i| i.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let inetervals = parse_intervals(&[[1, 2]]);
        assert_eq!(vec![-1], Solution::find_right_interval(inetervals));
    }

    #[test]
    fn case2() {
        let inetervals = parse_intervals(&[[3, 4], [2, 3], [1, 2]]);
        assert_eq!(vec![-1, 0, 1], Solution::find_right_interval(inetervals));
    }
}
