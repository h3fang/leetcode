pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let (mut pre, mut result) = (0, 0);
        for r in bank {
            let c = r.as_bytes().iter().filter(|&&c| c == b'1').count() as i32;
            if c > 0 {
                result += pre * c;
                pre = c;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bank = ["011001", "000000", "010100", "001000"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(8, Solution::number_of_beams(bank));
    }

    #[test]
    fn case2() {
        let bank = ["000", "111", "000"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(0, Solution::number_of_beams(bank));
    }
}
