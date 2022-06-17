pub struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut zero = arr.iter().filter(|&&a| a == 0).count();
        for i in (0..n).rev() {
            if arr[i] == 0 {
                zero -= 1;
            }
            if i + zero < n {
                arr[i + zero] = arr[i];
                if arr[i] == 0 && i + zero + 1 < n {
                    arr[i + zero + 1] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(vec![1, 0, 0, 2, 3, 0, 0, 4], arr);
    }
}
