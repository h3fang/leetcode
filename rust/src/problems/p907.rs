pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut s = vec![];
        let mut result = 0;

        for (r, &n) in arr.iter().chain(&[0]).enumerate() {
            while !s.is_empty() && arr[*s.last().unwrap()] > n {
                let j = s.pop().unwrap() as i64;
                let l = if s.is_empty() {
                    -1
                } else {
                    *s.last().unwrap() as i64
                };
                result = (result + (j - l) * (r as i64 - j) * arr[j as usize] as i64) % MOD;
            }
            s.push(r);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(17, Solution::sum_subarray_mins(vec![3, 1, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(444, Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]));
    }
}
