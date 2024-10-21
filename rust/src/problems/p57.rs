pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        let mut r = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            let intv = intervals[i].clone();
            if intv[1] < new_interval[0] {
                r.push(intv);
                i += 1;
            } else {
                break;
            }
        }

        while i < n {
            let intv = intervals[i].clone();
            if intv[0] <= new_interval[1] {
                new_interval[0] = new_interval[0].min(intv[0]);
                new_interval[1] = new_interval[1].max(intv[1]);
                i += 1;
            } else {
                break;
            }
        }

        r.push(new_interval);
        r.extend_from_slice(&intervals[i..]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let expected = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn case2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn case3() {
        let intervals = Vec::new();
        let new_interval = vec![5, 7];
        let expected = vec![vec![5, 7]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }
}
