pub struct Solution;

#[inline(always)]
fn intersect(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    a1.max(b1) < a2.min(b2)
}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        intersect(rec1[0], rec1[2], rec2[0], rec2[2])
            && intersect(rec1[1], rec1[3], rec2[1], rec2[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![1, 1, 3, 3]
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1]
        ));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3]
        ));
    }
}
