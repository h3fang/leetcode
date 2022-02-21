pub struct Solution;

impl Solution {
    pub fn push_dominoes(mut dominoes: String) -> String {
        let ds = unsafe { dominoes.as_bytes_mut() };
        let n = ds.len();
        let mut i = 0;
        let mut left = b'L';
        while i < n {
            let mut j = i;
            while j < n && ds[j] == b'.' {
                j += 1;
            }
            let right = if j < n { ds[j] } else { b'R' };
            if left == right {
                while i < j {
                    ds[i] = right;
                    i += 1;
                }
            } else if left == b'R' && right == b'L' {
                let mut k = j - 1;
                while i < k {
                    ds[i] = b'R';
                    i += 1;
                    ds[k] = b'L';
                    k -= 1;
                }
            }
            left = right;
            i = j + 1;
        }
        dominoes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("RR.L", Solution::push_dominoes("RR.L".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "LL.RR.LLRRLL..",
            Solution::push_dominoes(".L.R...LR..L..".to_string())
        );
    }
}
