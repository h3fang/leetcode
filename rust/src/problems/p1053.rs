pub struct Solution;

impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        for (i, &a) in arr.iter().enumerate().rev().skip(1) {
            if a > arr[i + 1] {
                let mut last = i + 1;
                let mut j = last + 1;
                while j < n && a > arr[j] {
                    if arr[j] != arr[last] {
                        last = j;
                    }
                    j += 1;
                }
                arr.swap(i, last);
                break;
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![3, 1, 2], Solution::prev_perm_opt1(vec![3, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 1, 5], Solution::prev_perm_opt1(vec![1, 1, 5]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![1, 7, 4, 6, 9],
            Solution::prev_perm_opt1(vec![1, 9, 4, 6, 7])
        );
    }
}
