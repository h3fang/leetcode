pub struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut apples: i32 = apple.into_iter().sum();
        capacity.sort_unstable_by_key(|e| -*e);
        for (i, x) in capacity.into_iter().enumerate() {
            apples -= x;
            if apples <= 0 {
                return i as i32 + 1;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]));
    }
}
