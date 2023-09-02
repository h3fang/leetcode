pub struct Solution;

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut pre = usize::MAX;
        for (i, &f) in forts.iter().enumerate() {
            if f == 1 || f == -1 {
                if pre < usize::MAX && forts[pre] != f {
                    result = result.max(i - pre - 1);
                }
                pre = i;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::capture_forts(vec![0, 0, 1, -1]));
    }
}
