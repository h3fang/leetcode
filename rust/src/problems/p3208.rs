pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let (mut result, mut c) = (0, 0);
        for i in 0..2 * n {
            if colors[i % n] != colors[(i + n - 1) % n] {
                c += 1;
            } else {
                c = 1;
            }
            if i >= n && c >= k {
                result += 1;
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
        assert_eq!(
            3,
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4)
        );
    }
}
