pub struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut r = 0;
        for _ in 0..32 {
            r <<= 1;
            r += x & 1;
            x >>= 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            964176192,
            Solution::reverse_bits(0b00000010100101000001111010011100)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::reverse_bits(0b10000000000000000000000000000000)
        );
    }
}
