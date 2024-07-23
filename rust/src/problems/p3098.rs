pub struct Solution;

use std::collections::HashMap;

const INF: i32 = i32::MAX;
const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut f: Vec<Vec<HashMap<i32, i64>>> = vec![vec![HashMap::new(); k as usize + 1]; n];
        let mut result = 0;
        for i in 0..n {
            f[i][1].insert(INF, 1);
            for j in 0..i {
                let diff = nums[i] - nums[j];
                for p in 2..=k as usize {
                    let m = &f[j][p - 1] as *const HashMap<i32, i64>;
                    for (&v, &c) in unsafe { &*m } {
                        let min = diff.min(v);
                        let e = f[i][p].entry(min).or_insert(0);
                        *e = (*e + c) % MOD;
                    }
                }
            }
            for (&v, c) in &f[i][k as usize] {
                result = (result + (v as i64 * c) % MOD) % MOD;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::sum_of_powers(vec![1, 2, 3, 4], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::sum_of_powers(vec![2, 2], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(10, Solution::sum_of_powers(vec![4, 3, -1], 2));
    }
}
