pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let num = num as u32;
        let mut mask = 1;
        while mask <= num {
            mask <<= 1;
        }
        ((!num) & (mask - 1)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_complement(5));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::find_complement(1));
    }
}
