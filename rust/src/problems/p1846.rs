pub struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut freq = vec![0; n + 1];
        for a in arr {
            freq[(a as usize).min(n)] += 1;
        }
        let mut result = 1;
        for (a, f) in freq.into_iter().enumerate().skip(2) {
            result = (a as i32).min(result + f);
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
            2,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            5,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5])
        );
    }
}
