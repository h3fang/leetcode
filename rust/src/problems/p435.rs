pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        let mut pre = 0;
        let mut result = 0;
        for (i, int) in intervals.iter().enumerate().skip(1) {
            if int[0] < intervals[pre][1] {
                result += 1;
                if int[1] < intervals[pre][1] {
                    pre = i;
                }
            } else {
                pre = i;
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
        let intervals = [[1, 2], [2, 3], [3, 4], [1, 3]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(1, Solution::erase_overlap_intervals(intervals));
    }

    #[test]
    fn case2() {
        let intervals = [[1, 2], [1, 2], [1, 2]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(2, Solution::erase_overlap_intervals(intervals));
    }

    #[test]
    fn case3() {
        let intervals = [[1, 2], [2, 3]].iter().map(|i| i.to_vec()).collect();
        assert_eq!(0, Solution::erase_overlap_intervals(intervals));
    }
}
