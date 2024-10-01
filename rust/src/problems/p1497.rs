pub struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut freq = vec![0; k as usize];
        for x in arr {
            freq[x.rem_euclid(k) as usize] += 1;
        }
        if freq[0] % 2 != 0 {
            return false;
        }
        for f in 1..(k + 1) / 2 {
            if freq[f as usize] != freq[(k - f) as usize] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_arrange(
            vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9],
            5
        ));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
    }

    #[test]
    fn case3() {
        assert!(!Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    }

    #[test]
    fn case4() {
        assert!(!Solution::can_arrange(
            vec![-1, -1, -1, -1, 2, 2, -2, -2],
            3
        ));
    }
}
