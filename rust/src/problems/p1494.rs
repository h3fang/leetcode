pub struct Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as u32;
        let m = 1 << n;
        let mut dp = vec![i32::MAX / 2; m];
        let mut need = vec![0; m];
        for e in &relations {
            need[1 << (e[1] - 1)] |= 1 << (e[0] - 1);
        }
        dp[0] = 0;
        for i in 1..m as i32 {
            need[i as usize] = need[(i & (i - 1)) as usize] | need[(i & (-i)) as usize];
            if need[i as usize] | i != i {
                continue;
            }
            let valid = i ^ need[i as usize];
            if valid.count_ones() <= k {
                dp[i as usize] = dp[i as usize].min(dp[(i ^ valid) as usize] + 1);
            } else {
                let mut sub = valid;
                while sub > 0 {
                    if sub.count_ones() <= k {
                        dp[i as usize] = dp[i as usize].min(dp[(i ^ sub) as usize] + 1);
                    }
                    sub = (sub - 1) & valid;
                }
            }
        }
        dp[m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let relations = [[2, 1], [3, 1], [1, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::min_number_of_semesters(4, relations, 2));
    }

    #[test]
    fn case2() {
        let relations = [[2, 1], [3, 1], [4, 1], [1, 5]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::min_number_of_semesters(5, relations, 2));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::min_number_of_semesters(11, vec![], 2));
    }
}
