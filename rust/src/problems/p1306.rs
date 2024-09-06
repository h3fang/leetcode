pub struct Solution;

impl Solution {
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        fn dfs(arr: &mut [i32], curr: i32) -> bool {
            if curr < 0 || curr >= arr.len() as i32 || arr[curr as usize] < 0 {
                false
            } else {
                let c = arr[curr as usize];
                if c == 0 {
                    return true;
                }
                arr[curr as usize] *= -1;
                dfs(arr, curr - c) || dfs(arr, curr + c)
            }
        }
        dfs(&mut arr, start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 5;
        assert!(Solution::can_reach(arr, start));
    }

    #[test]
    fn case2() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 0;
        assert!(Solution::can_reach(arr, start));
    }

    #[test]
    fn case3() {
        let arr = vec![3, 0, 2, 1, 2];
        let start = 2;
        assert!(!Solution::can_reach(arr, start));
    }
}
