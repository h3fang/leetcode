pub struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; heights.len()];
        let mut s = Vec::with_capacity(heights.len());
        for (i, &h) in heights.iter().enumerate().rev() {
            while !s.is_empty() && heights[*s.last().unwrap()] < h {
                s.pop();
                result[i] += 1;
            }
            if !s.is_empty() {
                result[i] += 1;
            }
            s.push(i);
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
            vec![3, 1, 2, 1, 1, 0],
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![4, 1, 1, 1, 0],
            Solution::can_see_persons_count(vec![5, 1, 2, 3, 10])
        );
    }
}
