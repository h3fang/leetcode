pub struct Solution;

impl Solution {
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        let (mut result, mut c) = (0, 0);
        for (i, (a, b)) in temperature_a.iter().zip(&temperature_b).enumerate().skip(1) {
            if a.cmp(&temperature_a[i - 1]) == b.cmp(&temperature_b[i - 1]) {
                c += 1;
            } else {
                result = result.max(c);
                c = 0;
            }
        }
        result.max(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::temperature_trend(vec![21, 18, 18, 18, 31], vec![34, 32, 16, 16, 17])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::temperature_trend(
                vec![5, 10, 16, -6, 15, 11, 3],
                vec![16, 22, 23, 23, 25, 3, -16]
            )
        );
    }
}
