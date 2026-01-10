pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let n = s2.len();
        let mut f = vec![0; n + 1];
        for x in s1.bytes() {
            let mut pre = 0;
            for (j, y) in s2.bytes().enumerate() {
                let tmp = f[j + 1];
                if x == y {
                    f[j + 1] = pre + x as i32;
                } else {
                    f[j + 1] = tmp.max(f[j]);
                }
                pre = tmp;
            }
        }
        s1.bytes().chain(s2.bytes()).map(|x| x as i32).sum::<i32>() - 2 * f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            231,
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            403,
            Solution::minimum_delete_sum("delete".to_string(), "leet".to_string())
        );
    }
}
