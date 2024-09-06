pub struct Solution;

impl Solution {
    pub fn verify_postorder_dc(postorder: Vec<i32>) -> bool {
        fn verify(postorder: &[i32], lb: i32, ub: i32) -> bool {
            let n = postorder.len();
            if n == 0 {
                return true;
            }
            let root = postorder[n - 1];
            if root <= lb || root >= ub {
                return false;
            }
            let mut i = n - 1;
            while i > 0 && postorder[i - 1] > root {
                if postorder[i - 1] >= ub {
                    return false;
                }
                i -= 1;
            }
            if !verify(&postorder[i..n - 1], root, ub) {
                return false;
            }
            verify(&postorder[..i], lb, root)
        }
        verify(&postorder, i32::MIN, i32::MAX)
    }

    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        let mut s = Vec::new();
        let mut root = i32::MAX;
        for &n in postorder.iter().rev() {
            if n > root {
                return false;
            }
            while !s.is_empty() && *s.last().unwrap() > n {
                root = s.pop().unwrap();
            }
            s.push(n);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::verify_postorder_dc(vec![1, 6, 3, 2, 5]));
        assert!(!Solution::verify_postorder(vec![1, 6, 3, 2, 5]));
    }

    #[test]
    fn case2() {
        assert!(Solution::verify_postorder_dc(vec![1, 3, 2, 6, 5]));
        assert!(Solution::verify_postorder(vec![1, 3, 2, 6, 5]));
    }
}
