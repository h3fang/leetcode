pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let (mut seats, mut last, mut ans) = (0, -1, 1);
        for (i, &c) in corridor.as_bytes().iter().enumerate() {
            if c == b'S' {
                seats += 1;
                if seats > 1 && seats % 2 == 1 {
                    ans = (ans * (i as i64 - last)) % MOD;
                }
                last = i as i64;
            }
        }
        if !(seats > 0 && seats % 2 == 0) {
            0
        } else {
            ans as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::number_of_ways("SSPPSPS".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::number_of_ways("PPSPSP".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::number_of_ways("S".to_string()));
    }
}
