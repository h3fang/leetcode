pub struct Solution;

impl Solution {
    pub fn kth_character(mut k: i64, operations: Vec<i32>) -> char {
        let mut c = 0;
        k -= 1;
        for i in (0..(64 - k.leading_zeros())).rev() {
            c += (k >> i) & (operations[i as usize] as i64);
        }
        (b'a' + (c % 26) as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!('a', Solution::kth_character(5, vec![0, 0, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!('b', Solution::kth_character(10, vec![0, 1, 0, 1]));
    }
}
