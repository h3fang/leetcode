pub struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        for (k, &c) in nums.iter().enumerate().skip(2).rev() {
            let (mut i, mut j) = (0, k - 1);
            while i < j {
                if nums[i] + nums[j] <= c {
                    i += 1;
                } else {
                    ans += j - i;
                    j -= 1;
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::triangle_number(vec![2, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::triangle_number(vec![4, 2, 3, 4]));
    }
}
