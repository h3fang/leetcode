pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let n = next_visit.len();
        let mut f = vec![0; n];
        for (i, &j) in next_visit[..n - 1].iter().enumerate() {
            f[i + 1] = (2 * f[i] - f[j as usize] + 2 + MOD) % MOD;
        }
        f[n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::first_day_been_in_all_rooms(vec![0, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::first_day_been_in_all_rooms(vec![0, 0, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::first_day_been_in_all_rooms(vec![0, 1, 2, 0]));
    }
}
