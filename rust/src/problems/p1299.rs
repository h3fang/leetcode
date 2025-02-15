pub struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for e in arr.iter_mut().rev() {
            (*e, max) = (max, max.max(*e));
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![18, 6, 6, 6, 1, -1],
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![-1], Solution::replace_elements(vec![400]));
    }
}
