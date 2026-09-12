pub struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut intervals = intervals
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e[1], e[0], e[2], i as i32))
            .collect::<Vec<_>>();
        intervals.sort_unstable_by_key(|e| e.0);

        let mut f = vec![vec![(0, vec![]); 5]; n + 1];

        for (i, &(_r, l, w, idx)) in intervals.iter().enumerate() {
            let k = intervals[..i].partition_point(|e| e.0 < l);
            for j in 1..5 {
                let s = f[k][j - 1].0 - i64::from(w);
                let mut ids = f[k][j - 1].1.clone();
                ids.push(idx);
                ids.sort_unstable();

                f[i + 1][j] = (f[i][j].clone()).min((s, ids));
            }
        }

        f[n][4].1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let intervals = [
            [1, 3, 2],
            [4, 5, 2],
            [1, 5, 5],
            [6, 9, 3],
            [6, 7, 1],
            [8, 9, 1],
        ]
        .iter()
        .map(|i| i.to_vec())
        .collect();
        assert_eq!(vec![2, 3], Solution::maximum_weight(intervals));
    }

    #[test]
    fn case2() {
        let intervals = [
            [5, 8, 1],
            [6, 7, 7],
            [4, 7, 3],
            [9, 10, 6],
            [7, 8, 2],
            [11, 14, 3],
            [3, 5, 5],
        ]
        .iter()
        .map(|i| i.to_vec())
        .collect();
        assert_eq!(vec![1, 3, 5, 6], Solution::maximum_weight(intervals));
    }
}
