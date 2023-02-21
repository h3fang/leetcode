pub struct Solution;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut right_most = vec![0; n as usize + 1];
        for (i, &r) in ranges.iter().enumerate() {
            let s = (i as i32 - r).max(0);
            let e = i as i32 + r;
            right_most[s as usize] = right_most[s as usize].max(e);
        }
        let mut result = 0;
        let mut pre = 0;
        let mut next = 0;
        for i in 0..n {
            next = next.max(right_most[i as usize]);
            if i == pre {
                if i == next {
                    return -1;
                }
                pre = next;
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_taps(3, vec![0, 0, 0, 0]));
    }
}
