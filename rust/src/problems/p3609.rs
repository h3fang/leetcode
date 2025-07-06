pub struct Solution;

impl Solution {
    pub fn min_moves(mut sx: i32, mut sy: i32, mut x: i32, mut y: i32) -> i32 {
        let mut ans = 0;
        while (sx, sy) != (x, y) {
            ans += 1;
            if x < sx || y < sy {
                return -1;
            }

            if x == y {
                if sy > 0 {
                    x = 0;
                } else {
                    y = 0;
                }
                continue;
            }

            if x < y {
                (x, y) = (y, x);
                (sx, sy) = (sy, sx);
            }

            if x < 2 * y {
                x -= y;
            } else if x % 2 == 0 {
                x /= 2;
            } else {
                return -1;
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
        assert_eq!(2, Solution::min_moves(1, 2, 5, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_moves(0, 1, 2, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_moves(1, 1, 2, 2));
    }
}
