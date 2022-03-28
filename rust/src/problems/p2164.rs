pub struct Solution;

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut even: Vec<i32> = nums.iter().step_by(2).cloned().collect();
        let mut odd: Vec<i32> = nums.iter().skip(1).step_by(2).cloned().collect();
        even.sort_unstable();
        odd.sort_unstable_by(|a, b| b.cmp(a));
        let mut result = Vec::with_capacity(nums.len());
        for (&a, &b) in even.iter().zip(odd.iter()) {
            result.push(a);
            result.push(b);
        }
        if even.len() > odd.len() {
            result.push(*even.last().unwrap());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 3, 4, 1], Solution::sort_even_odd(vec![4, 1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![2, 1], Solution::sort_even_odd(vec![2, 1]));
    }
}
