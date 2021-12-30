pub struct Solution;

impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut r = 0;
        for i in 0..k {
            let mut len = 0;
            let mut d = vec![];
            for &n in arr[i..].iter().step_by(k) {
                len += 1;
                if d.is_empty() || n >= *d.last().unwrap() {
                    d.push(n);
                } else {
                    let mut a = 0;
                    let mut b = d.len();
                    while a < b {
                        let mid = a + (b - a) / 2;
                        if d[mid] <= n {
                            a = mid + 1;
                        } else {
                            b = mid;
                        }
                    }
                    d[a] = n;
                }
            }
            r += len - d.len();
        }
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::k_increasing(vec![5, 4, 3, 2, 1], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::k_increasing(vec![4, 1, 5, 2, 6, 2], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::k_increasing(vec![4, 1, 5, 2, 6, 2], 3));
    }

    #[test]
    fn case4() {
        assert_eq!(
            12,
            Solution::k_increasing(
                vec![12, 6, 12, 6, 14, 2, 13, 17, 3, 8, 11, 7, 4, 11, 18, 8, 8, 3],
                1
            )
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            4,
            Solution::k_increasing(vec![2, 2, 2, 2, 2, 1, 1, 4, 4, 3, 3, 3, 3, 3], 1)
        );
    }
}
