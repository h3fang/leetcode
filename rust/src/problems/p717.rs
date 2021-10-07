pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();
        let mut i = 0;
        while i < n - 1 {
            if bits[i] == 1 {
                i += 2;
            } else {
                i += 1;
            }
        }

        i == n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bits = vec![1, 0, 0];
        assert_eq!(true, Solution::is_one_bit_character(bits));
    }

    #[test]
    fn case2() {
        let bits = vec![1, 1, 1, 0];
        assert_eq!(false, Solution::is_one_bit_character(bits));
    }
}
