pub struct Solution;

fn is_prime(x: i32) -> bool {
    if x == 1 {
        return false;
    }
    for i in 2..=((x as f64).sqrt().round() as i32) {
        if x % i == 0 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for (i, r) in nums.iter().enumerate() {
            for j in [i, n - 1 - i] {
                if is_prime(r[j]) {
                    ans = ans.max(r[j]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [[1, 2, 3], [5, 6, 7], [9, 10, 11]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(11, Solution::diagonal_prime(nums));
    }

    #[test]
    fn case2() {
        let nums = [[1, 2, 3], [5, 17, 7], [9, 11, 10]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(17, Solution::diagonal_prime(nums));
    }

    #[test]
    fn case3() {
        let nums = [
            [367, 941, 856, 950, 80, 419],
            [56, 452, 364, 619, 924, 636],
            [775, 262, 689, 318, 577, 870],
            [824, 465, 423, 186, 32, 465],
            [822, 271, 196, 973, 397, 66],
            [367, 99, 127, 27, 360, 22],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(419, Solution::diagonal_prime(nums));
    }
}
