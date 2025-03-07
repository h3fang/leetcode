pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            i: usize,
            sum: i32,
            min_sum: i32,
            max_sum: i32,
            n1: &str,
            n2: &str,
            lb: bool,
            ub: bool,
            cache: &mut [Vec<i32>],
        ) -> i32 {
            if sum > max_sum {
                return 0;
            }
            if i == n1.len() {
                return i32::from(sum >= min_sum);
            }
            if !lb && !ub && cache[i][sum as usize] != -1 {
                return cache[i][sum as usize];
            }
            let max = if ub { n2.as_bytes()[i] - b'0' } else { 9 } as i32;
            let min = if lb { n1.as_bytes()[i] - b'0' } else { 0 } as i32;
            let mut r = 0;
            for d in min..=max {
                r = (r + dfs(
                    i + 1,
                    sum + d,
                    min_sum,
                    max_sum,
                    n1,
                    n2,
                    lb && d == min,
                    ub && d == max,
                    cache,
                )) % MOD;
            }
            if !lb && !ub {
                cache[i][sum as usize] = r;
            }
            r
        }
        let mut cache = vec![vec![-1; max_sum as usize + 1]; num2.len()];
        let num1 = "0".repeat(num2.len() - num1.len()) + &num1;
        dfs(0, 0, min_sum, max_sum, &num1, &num2, true, true, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(11, Solution::count("1".to_string(), "12".to_string(), 1, 8));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count("1".to_string(), "5".to_string(), 1, 5));
    }
}
