pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        fn dfs(
            locations: &[i32],
            f: &mut [Vec<i32>],
            loc: usize,
            target: usize,
            fuel: usize,
        ) -> i32 {
            if f[loc][fuel] != -1 {
                return f[loc][fuel];
            }
            f[loc][fuel] = 0;
            if (locations[loc] - locations[target]).unsigned_abs() as usize > fuel {
                return 0;
            }
            for (i, &l) in locations.iter().enumerate() {
                if loc == i {
                    continue;
                }
                let cost = (locations[loc] - l).unsigned_abs() as usize;
                if cost <= fuel {
                    f[loc][fuel] = (f[loc][fuel] + dfs(locations, f, i, target, fuel - cost)) % MOD;
                }
            }
            if loc == target {
                f[loc][fuel] = (1 + f[loc][fuel]) % MOD;
            }
            f[loc][fuel]
        }
        let mut f = vec![vec![-1; fuel as usize + 1]; locations.len()];
        dfs(
            &locations,
            &mut f,
            start as usize,
            finish as usize,
            fuel as usize,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_routes(vec![4, 3, 1], 1, 0, 6));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::count_routes(vec![5, 2, 1], 0, 2, 3));
    }
}
