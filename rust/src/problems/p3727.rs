pub struct Solution;

impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        let n: usize = nums.len();
        let (lesser, median, greater) = nums.select_nth_unstable_by_key(n / 2, |x| x.abs());

        let mut ans = (*median * *median) as i64;

        for &mut x in greater {
            ans += (x * x) as i64;
        }

        for &mut x in lesser {
            ans -= (x * x) as i64;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::max_alternating_sum(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::max_alternating_sum(vec![1, -1, 2, -2, 3, -3]));
    }
}
