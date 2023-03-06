pub struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut c = 0;
        let mut min = 1;
        for n in arr {
            if n != min {
                let diff = n - min;
                if c + diff >= k {
                    return min + k - c - 1;
                }
                c += diff;
            }
            min = n + 1;
        }
        min + k - c - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::find_kth_positive(vec![1, 2, 3, 4], 2));
    }
}
