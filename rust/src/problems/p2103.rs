pub struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut poles = [(false, false, false); 10];
        for w in rings.as_bytes().windows(2) {
            if w[0] == b'R' {
                poles[(w[1] - b'0') as usize].0 = true;
            } else if w[0] == b'G' {
                poles[(w[1] - b'0') as usize].1 = true;
            } else if w[0] == b'B' {
                poles[(w[1] - b'0') as usize].2 = true;
            }
        }
        poles.iter().filter(|p| *p == &(true, true, true)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::count_points("B0B6G0R6R0R6G9".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_points("B0R0G0R9R0B0G0".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::count_points("G4".to_string()));
    }
}
