pub struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut s: Vec<(i32, i32)> = vec![];
        for &n in nums.iter().rev() {
            let mut c = 0i32;
            while let Some(&t) = s.last() {
                if t.0 < n {
                    c = (c + 1).max(t.1);
                    s.pop();
                } else {
                    break;
                }
            }
            s.push((n, c));
        }
        s.iter().map(|t| t.1).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::total_steps(vec![4, 5, 7, 7, 13]));
    }
}
