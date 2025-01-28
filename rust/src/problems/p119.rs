pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut r = vec![0; n + 1];
        r[0] = 1;
        for i in 1..=n {
            r[i] = (r[i - 1] as usize * (n + 1 - i) / i) as i32;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1], Solution::get_row(0));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![1, 1], Solution::get_row(1));
    }
}
