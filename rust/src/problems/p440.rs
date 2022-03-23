pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        fn get_subtree(curr: i32, n: i64) -> i32 {
            let mut result = 0;
            let mut first = curr as i64;
            let mut last = curr as i64;
            while first <= n {
                result += last.min(n) - first + 1;
                first *= 10;
                last = last * 10 + 9;
            }
            result as i32
        }

        let mut curr = 1;
        k -= 1;
        while k > 0 {
            let c = get_subtree(curr, n as i64);
            if c <= k {
                curr += 1;
                k -= c;
            } else {
                curr *= 10;
                k -= 1;
            }
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::find_kth_number(13, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_kth_number(1, 1));
    }
}
