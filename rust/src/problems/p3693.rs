pub struct Solution;

impl Solution {
    pub fn climb_stairs(_n: i32, costs: Vec<i32>) -> i32 {
        let (mut f0, mut f1, mut f2) = (0, 0, 0);

        for c in costs {
            (f0, f1, f2) = (f1, f2, (f0 + 9).min(f1 + 4).min(f2 + 1) + c);
        }

        f2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::climb_stairs(4, vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::climb_stairs(4, vec![5, 1, 6, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(12, Solution::climb_stairs(3, vec![9, 8, 3]));
    }
}
