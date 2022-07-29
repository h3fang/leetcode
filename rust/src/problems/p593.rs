pub struct Solution;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        fn check_mid(p1: &[i32], p2: &[i32], p3: &[i32], p4: &[i32]) -> bool {
            p1[0] + p2[0] == p3[0] + p4[0] && p1[1] + p2[1] == p3[1] + p4[1]
        }
        fn check_len(a: (i32, i32), b: (i32, i32)) -> bool {
            let l1 = a.0 * a.0 + a.1 * a.1;
            let l2 = b.0 * b.0 + b.1 * b.1;
            l1 > 0 && l1 == l2
        }
        fn check_angle(a: (i32, i32), b: (i32, i32)) -> bool {
            a.0 * b.0 + a.1 * b.1 == 0
        }
        fn is_square(p1: &[i32], p2: &[i32], p3: &[i32], p4: &[i32]) -> bool {
            let a = (p2[0] - p1[0], p2[1] - p1[1]);
            let b = (p4[0] - p3[0], p4[1] - p3[1]);
            check_mid(p1, p2, p3, p4) && check_len(a, b) && check_angle(a, b)
        }

        is_square(&p1, &p2, &p3, &p4)
            || is_square(&p1, &p3, &p2, &p4)
            || is_square(&p1, &p4, &p3, &p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 1];
        assert_eq!(true, Solution::valid_square(p1, p2, p3, p4));
    }

    #[test]
    fn case2() {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 12];
        assert_eq!(false, Solution::valid_square(p1, p2, p3, p4));
    }
}
