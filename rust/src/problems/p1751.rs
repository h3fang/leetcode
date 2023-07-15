pub struct Solution;

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_unstable_by_key(|e| e[1]);
        let k = k as usize;
        let n = events.len();
        let mut f = vec![vec![0; k + 1]; n + 1];
        for (i, a) in events.iter().enumerate() {
            let p = events[..i].partition_point(|b| b[1] < a[0]);
            for j in 1..=k {
                f[i + 1][j] = f[i][j].max(f[p][j - 1] + a[2]);
            }
        }
        f[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let events = [[1, 2, 4], [3, 4, 3], [2, 3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(7, Solution::max_value(events, 2));
    }

    #[test]
    fn case2() {
        let events = [[1, 2, 4], [3, 4, 3], [2, 3, 10]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(10, Solution::max_value(events, 2));
    }

    #[test]
    fn case3() {
        let events = [[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(9, Solution::max_value(events, 3));
    }
}
