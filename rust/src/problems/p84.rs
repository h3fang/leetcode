pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        let mut result = 0;
        let mut stack = vec![(-1, 0)];

        for (i, h) in heights.iter().enumerate() {
            while *h < stack.last().unwrap().1 {
                let height = stack.pop().unwrap().1;
                let width = i as i32 - stack.last().unwrap().0 - 1;
                result = result.max(height * width);
            }
            stack.push((i as i32, *h));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(10, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn case2() {
        let heights = vec![2, 4];
        assert_eq!(4, Solution::largest_rectangle_area(heights));
    }
}
