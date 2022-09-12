pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        fn query(f: &[i32], i: usize, lb: i32, rb: i32, l: i32, r: i32) -> i32 {
            if l <= lb && r >= rb {
                return f[i];
            }
            let mut result = 0;
            let m = lb + (rb - lb) / 2;
            if l <= m {
                result = query(f, i * 2, lb, m, l, r);
            }
            if r > m {
                result = result.max(query(f, 2 * i + 1, m + 1, rb, l, r));
            }
            result
        }

        fn update(f: &mut [i32], i: usize, lb: i32, rb: i32, p: i32, v: i32) {
            if lb == rb {
                f[i] = v;
                return;
            }
            let m = lb + (rb - lb) / 2;
            if p <= m {
                update(f, 2 * i, lb, m, p, v);
            } else {
                update(f, 2 * i + 1, m + 1, rb, p, v);
            }
            f[i] = f[2 * i].max(f[2 * i + 1]);
        }

        let max = nums.iter().max().cloned().unwrap();
        let mut f = vec![0; max as usize * 4];
        for n in nums {
            if n == 1 {
                update(&mut f, 1, 1, max, 1, 1);
            } else {
                let r = 1 + query(&f, 1, 1, max, 1.max(n - k), n - 1);
                update(&mut f, 1, 1, max, n, r);
            }
        }
        f[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::length_of_lis(vec![1, 5, 1], 1));
    }
}
