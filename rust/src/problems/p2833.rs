pub struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut f = [0i32; 2];
        for b in moves.bytes() {
            match b {
                b'L' => f[0] += 1,
                b'R' => f[1] += 1,
                _ => {}
            }
        }
        let both = moves.len() as i32 - f[0] - f[1];
        (f[0] - f[1]).abs() + both
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::furthest_distance_from_origin("L_RL__R".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::furthest_distance_from_origin("_R__LL_".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            7,
            Solution::furthest_distance_from_origin("_______".to_string())
        );
    }
}
