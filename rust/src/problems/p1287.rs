pub struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        for a in [arr[n / 4], arr[n / 2], arr[3 * n / 4]] {
            let l = arr.partition_point(|&e| e < a);
            let r = arr.partition_point(|&e| e <= a);
            if (r - l) * 4 > n {
                return a;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            6,
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_special_integer(vec![1, 1]));
    }
}
