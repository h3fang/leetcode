pub struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (k, n) = (k as usize, nums.len());
        let mut s = Vec::with_capacity(n);
        for (i, &x) in nums.iter().enumerate() {
            while !s.is_empty() && *s.last().unwrap() > x && s.len() + n - i > k {
                s.pop();
            }
            s.push(x);
        }
        s.resize(k, 0);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 6], Solution::most_competitive(vec![3, 5, 2, 6], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![2, 3, 3, 4],
            Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4)
        );
    }
}
