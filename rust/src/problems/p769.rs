pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max = -1;
        for (i, &e) in arr.iter().enumerate() {
            max = max.max(e);
            if i as i32 == max {
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
        assert_eq!(1, Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]));
    }
}
