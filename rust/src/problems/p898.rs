pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(mut arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut s = HashSet::with_capacity(n);
        for i in 0..n {
            let x = arr[i];
            s.insert(x);
            for j in (0..i).rev() {
                if x | arr[j] == arr[j] {
                    break;
                }
                arr[j] |= x;
                s.insert(arr[j]);
            }
        }
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::subarray_bitwise_o_rs(vec![0]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::subarray_bitwise_o_rs(vec![1, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::subarray_bitwise_o_rs(vec![1, 2, 4]));
    }
}
