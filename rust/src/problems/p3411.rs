pub struct Solution;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let (mut prod, mut l, mut ans) = (1, 0, 2);

        for (r, &x) in nums.iter().enumerate() {
            while gcd(prod, x) > 1 {
                prod /= nums[l];
                l += 1;
            }
            prod *= x;
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_length(vec![2, 3, 4, 5, 6]));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::max_length(vec![1, 2, 3, 1, 4, 5, 1]));
    }
}
