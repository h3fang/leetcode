pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut rows: Vec<usize> = (0..strs.len() - 1).collect();
        for j in 0..strs[0].len() {
            let mut done = true;
            for &i in &rows {
                if strs[i].as_bytes()[j] > strs[i + 1].as_bytes()[j] {
                    ans += 1;
                    done = false;
                    break;
                }
            }
            if !done {
                continue;
            }
            let mut c = 0;
            for i in 0..rows.len() {
                let i = rows[i];
                if strs[i].as_bytes()[j] == strs[i + 1].as_bytes()[j] {
                    rows[c] = i;
                    c += 1;
                }
            }
            rows.resize(c, 0);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = ["ca", "bb", "ac"].iter().map(|s| s.to_string()).collect();
        assert_eq!(1, Solution::min_deletion_size(strs));
    }

    #[test]
    fn case2() {
        let strs = ["xc", "yb", "za"].iter().map(|s| s.to_string()).collect();
        assert_eq!(0, Solution::min_deletion_size(strs));
    }

    #[test]
    fn case3() {
        let strs = ["zyx", "wvu", "tsr"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(3, Solution::min_deletion_size(strs));
    }
}
