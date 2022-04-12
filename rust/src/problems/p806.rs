pub struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 0;
        let mut current = 0;
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            if current + widths[i] > 100 {
                lines += 1;
                current = widths[i];
            } else {
                current += widths[i];
            }
        }
        vec![lines + 1, current]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let widths = vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];
        assert_eq!(
            vec![3, 60],
            Solution::number_of_lines(widths, "abcdefghijklmnopqrstuvwxyz".to_string())
        );
    }

    #[test]
    fn case2() {
        let widths = vec![
            4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];
        assert_eq!(
            vec![2, 4],
            Solution::number_of_lines(widths, "bbbcccdddaaa".to_string())
        );
    }
}
