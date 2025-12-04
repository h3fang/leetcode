pub struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let (mut right, mut static_, mut ans) = (0, false, 0);
        for b in directions.bytes() {
            match b {
                b'L' => {
                    if static_ {
                        ans += 1;
                    } else if right > 0 {
                        ans += 1 + right;
                        right = 0;
                        static_ = true;
                    }
                }
                b'R' => {
                    static_ = false;
                    right += 1;
                }
                b'S' => {
                    ans += right;
                    right = 0;
                    static_ = true;
                }
                _ => unreachable!(),
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_collisions("RLRSLL".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_collisions("LLRR".to_string()));
    }
}
