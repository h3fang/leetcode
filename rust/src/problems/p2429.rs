pub struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let n1 = num1.count_ones();
        let n2 = num2.count_ones();
        match n1.cmp(&n2) {
            std::cmp::Ordering::Less => {
                let mut n = num1;
                let mut bits = 0;
                for i in 0..32 {
                    if num1 & (1 << i) == 0 {
                        n |= 1 << i;
                        bits += 1;
                    }
                    if bits == (n2 - n1) {
                        break;
                    }
                }
                n
            }
            std::cmp::Ordering::Equal => num1,
            std::cmp::Ordering::Greater => {
                let mut n = num1;
                let mut bits = 0;
                for i in 0..32 {
                    if (1 << i) & n > 0 {
                        n &= !(1 << i);
                        bits += 1;
                    }
                    if bits == (n1 - n2) {
                        break;
                    }
                }
                n
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimize_xor(3, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimize_xor(1, 12));
    }

    #[test]
    fn case3() {
        assert_eq!(67, Solution::minimize_xor(65, 84));
    }
}
