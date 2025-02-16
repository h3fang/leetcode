pub struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        fn dfs(n: i32, rem: u32, arr: &mut [i32], i: usize) -> bool {
            if rem == 0 || i == 2 * n as usize - 1 {
                return true;
            }
            if arr[i] != 0 {
                return dfs(n, rem, arr, i + 1);
            }
            for x in (1..=n).rev() {
                if rem & (1 << x) == 0 {
                    continue;
                }
                if x == 1 {
                    arr[i] = 1;
                    if dfs(n, rem & !(1 << x), arr, i + 1) {
                        return true;
                    }
                    arr[i] = 0;
                } else {
                    let j = i + x as usize;
                    if j >= 2 * n as usize - 1 || arr[j] != 0 {
                        continue;
                    } else {
                        arr[i] = x;
                        arr[j] = x;
                        if dfs(n, rem & !(1 << x), arr, i + 1) {
                            return true;
                        }
                        arr[i] = 0;
                        arr[j] = 0;
                    }
                }
            }
            false
        }
        let mut result = vec![0; 2 * n as usize - 1];
        dfs(n, (1 << (n + 1)) - 2, &mut result, 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 1, 2, 3, 2],
            Solution::construct_distanced_sequence(3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2],
            Solution::construct_distanced_sequence(5)
        );
    }
}
