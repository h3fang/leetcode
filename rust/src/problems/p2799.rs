pub struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut s = vec![0; 2001];
        let mut n = 0;
        for &x in &nums {
            if s[x as usize] == 0 {
                n += 1;
            }
            s[x as usize] += 1;
        }
        let mut ans = 0;
        s.iter_mut().for_each(|x| *x = 0);
        let mut l = 0;
        for &x in &nums {
            if s[x as usize] == 0 {
                n -= 1;
            }
            s[x as usize] += 1;
            while n <= 0 {
                s[nums[l] as usize] -= 1;
                if s[nums[l] as usize] == 0 {
                    n += 1;
                }
                l += 1;
            }
            ans += l as i32;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_complete_subarrays(vec![5, 5, 5, 5]));
    }
}
