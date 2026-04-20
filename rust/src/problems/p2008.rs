pub struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut f = vec![0; n + 1];
        let mut m = vec![vec![]; n + 1];
        for r in &rides {
            m[r[1] as usize].push(r);
        }
        for i in 1..=n {
            f[i] = f[i - 1];
            for r in &m[i] {
                f[i] = f[i].max(f[r[0] as usize] + (r[1] - r[0] + r[2]) as i64);
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rides = [[2, 5, 4], [1, 5, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(7, Solution::max_taxi_earnings(5, rides));
    }

    #[test]
    fn case2() {
        let rides = [
            [1, 6, 1],
            [3, 10, 2],
            [10, 12, 3],
            [11, 12, 2],
            [12, 15, 2],
            [13, 18, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(20, Solution::max_taxi_earnings(20, rides));
    }
}
