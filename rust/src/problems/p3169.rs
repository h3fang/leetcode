pub struct Solution;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable();
        let (mut ans, mut pre) = (0, 0);
        for m in meetings {
            if m[0] > pre {
                ans += m[0] - pre - 1;
                pre = m[1];
            } else {
                pre = pre.max(m[1]);
            }
        }
        ans + days - pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let meetings = [[5, 7], [1, 3], [9, 10]]
            .iter()
            .map(|m| m.to_vec())
            .collect();
        assert_eq!(2, Solution::count_days(10, meetings));
    }

    #[test]
    fn case2() {
        let meetings = [[2, 4], [1, 3]].iter().map(|m| m.to_vec()).collect();
        assert_eq!(1, Solution::count_days(5, meetings));
    }

    #[test]
    fn case3() {
        let meetings = [[1, 6]].iter().map(|m| m.to_vec()).collect();
        assert_eq!(0, Solution::count_days(6, meetings));
    }
}
