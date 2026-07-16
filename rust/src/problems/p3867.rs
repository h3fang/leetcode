pub struct Solution;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}

impl Solution {
    pub fn gcd_sum(mut nums: Vec<i32>) -> i64 {
        let mut max = i32::MIN;
        for x in &mut nums {
            max = (*x).max(max);
            *x = gcd(*x, max);
        }

        nums.sort_unstable();

        let n = nums.len();
        let mut ans = 0;
        for (i, &a) in nums.iter().enumerate().take(n / 2) {
            ans += gcd(a, nums[n - 1 - i]) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::gcd_sum(vec![2, 6, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::gcd_sum(vec![3, 6, 2, 8]));
    }
}
