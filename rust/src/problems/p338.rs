pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut r = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            r[i] = r[i >> 1] + i as i32 % 2;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    }
}
