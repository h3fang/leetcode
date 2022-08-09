pub struct Solution;

use std::collections::HashMap;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut m = HashMap::new();
        for &n in &arr {
            m.insert(n as i64, 1);
        }
        let mut result = arr.len() as i64;
        for (i, &n) in arr.iter().enumerate() {
            let n = n as i64;
            for &x in &arr[..i] {
                let x = x as i64;
                if n % x != 0 {
                    continue;
                }
                let y = n / x;
                let c1 = *m.get(&x).unwrap();
                if let Some(&c2) = m.get(&y) {
                    let e = m.entry(n).or_insert(0);
                    *e = (*e + c1 * c2) % MOD;
                    result = (result + c1 * c2) % MOD;
                }
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
        assert_eq!(3, Solution::num_factored_binary_trees(vec![2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::num_factored_binary_trees(vec![2, 4, 5, 10]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            5,
            Solution::num_factored_binary_trees(vec![15, 13, 22, 7, 11])
        );
    }
}
