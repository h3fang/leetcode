pub struct Solution;

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        fn inside(end1: i32, end2: i32, x: i32) -> bool {
            (end1.min(end2)..=end1.max(end2)).contains(&x)
        }

        if (a == e && (c != a || !inside(b, f, d))) || (b == f && (d != b || !inside(a, e, c))) {
            return 1;
        }

        let (i1, j1, i2, j2, i3, j3) = (a + b, a - b, c + d, c - d, e + f, e - f);
        if (i2 == i3 && (i1 != i2 || !inside(j2, j3, j1)))
            || (j2 == j3 && (j1 != j2 || !inside(i2, i3, i1)))
        {
            return 1;
        };
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            1,
            Solution::min_moves_to_capture_the_queen(5, 5, 2, 2, 1, 1)
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            1,
            Solution::min_moves_to_capture_the_queen(1, 6, 3, 3, 5, 6)
        );
    }
}
