pub struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut result = 0;
        for j in (1..n).rev() {
            if colors[j] != colors[0] {
                result = result.max(j);
                break;
            }
        }
        for j in 0..n - 1 {
            if colors[n - 1] != colors[j] {
                result = result.max(n - 1 - j);
                break;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_distance(vec![1, 8, 3, 8, 3]));
    }
}
