pub struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut m = arr1.len();
        let mut n = arr2.len();
        let mut result = Vec::with_capacity(m.max(n) + 1);
        let mut c = 0;
        while m > 0 || n > 0 || c != 0 {
            let a = if m > 0 {
                m -= 1;
                arr1[m]
            } else {
                0
            };
            let b = if n > 0 {
                n -= 1;
                arr2[n]
            } else {
                0
            };
            let x = a + b + c;
            match x {
                0 | 1 => {
                    result.push(x);
                    c = 0;
                }
                2 | 3 => {
                    result.push(x - 2);
                    c = -1;
                }
                -1 => {
                    result.push(1);
                    c = 1;
                }
                _ => unreachable!(),
            }
        }
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 0, 0, 0, 0],
            Solution::add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0], Solution::add_negabinary(vec![0], vec![0]));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![1], Solution::add_negabinary(vec![0], vec![1]));
    }
}
