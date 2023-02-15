pub struct Solution;

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(num.len());
        let mut c = 0;
        while k != 0 || c != 0 || !num.is_empty() {
            let d = k % 10 + c + num.pop().unwrap_or(0);
            result.push(d % 10);
            c = d / 10;
            k /= 10;
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![4, 5, 5],
            Solution::add_to_array_form(vec![2, 7, 4], 181)
        );
    }
}
