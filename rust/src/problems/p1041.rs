pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut x, mut y) = (0, 0);
        let mut d = 0;
        for inst in instructions.chars() {
            match inst {
                'G' => {
                    x += DIRS[d].0;
                    y += DIRS[d].1
                }
                'L' => {
                    d = (d + 4 - 1) % 4;
                }
                'R' => {
                    d = (d + 1) % 4;
                }
                _ => {}
            }
        }
        d != 0 || (x, y) == (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_robot_bounded("GGLLGG".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_robot_bounded("GG".to_string()));
    }

    #[test]
    fn case3() {
        assert!(Solution::is_robot_bounded("GL".to_string()));
    }
}
