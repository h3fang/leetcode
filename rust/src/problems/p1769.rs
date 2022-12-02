pub struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut result = vec![0; boxes.len()];
        let mut count = 0;
        let mut pre = 0;
        for (i, c) in boxes.char_indices() {
            pre += count;
            result[i] = pre;
            if c == '1' {
                count += 1;
            }
        }
        count = 0;
        pre = 0;
        for (i, c) in boxes.char_indices().rev() {
            pre += count;
            result[i] += pre;
            if c == '1' {
                count += 1;
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
        assert_eq!(vec![1, 1, 3], Solution::min_operations("110".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![11, 8, 5, 4, 3, 4],
            Solution::min_operations("001011".to_string())
        );
    }
}
