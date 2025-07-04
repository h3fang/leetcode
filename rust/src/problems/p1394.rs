pub struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut f = vec![0; 501];
        for x in arr {
            f[x as usize] += 1;
        }
        let mut ans = -1;
        for (i, v) in f.into_iter().enumerate().skip(1) {
            if i as i32 == v {
                ans = ans.max(v);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_lucky(vec![2, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::find_lucky(vec![2, 2, 2, 3, 3]));
    }

    #[test]
    fn case4() {
        assert_eq!(-1, Solution::find_lucky(vec![5]));
    }

    #[test]
    fn case5() {
        assert_eq!(7, Solution::find_lucky(vec![7, 7, 7, 7, 7, 7, 7]));
    }
}
