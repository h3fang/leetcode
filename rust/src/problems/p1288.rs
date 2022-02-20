pub struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|i| (i[0], -i[1]));
        let mut result = 0;
        let mut right = 0;
        for i in intervals {
            if i[1] > right {
                result += 1;
                right = i[1];
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
        let intervals = [[1, 4], [3, 6], [2, 8]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        assert_eq!(2, Solution::remove_covered_intervals(intervals));
    }

    #[test]
    fn case2() {
        let intervals = [[1, 4], [2, 3]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        assert_eq!(1, Solution::remove_covered_intervals(intervals));
    }

    #[test]
    fn case3() {
        let intervals = [[1, 2], [1, 4], [3, 4]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        assert_eq!(1, Solution::remove_covered_intervals(intervals));
    }
}
