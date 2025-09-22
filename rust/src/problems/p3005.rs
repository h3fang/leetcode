pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut f = [0; 101];
        let (mut max, mut ans) = (0, 0);
        for x in nums {
            f[x as usize] += 1;
            if f[x as usize] > max {
                max = f[x as usize];
                ans = max;
            } else if f[x as usize] == max {
                ans += max;
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
        assert_eq!(4, Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]));
    }
}
