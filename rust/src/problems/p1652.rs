pub struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut result = vec![0; n];
        if k == 0 {
            return result;
        }
        let mut sum = if k > 0 {
            code[1..1 + k as usize].iter().sum::<i32>()
        } else {
            code[n - k.unsigned_abs() as usize..].iter().sum::<i32>()
        };
        for (i, r) in result.iter_mut().enumerate() {
            *r = sum;
            if k > 0 {
                let j = (i + 1) % n;
                sum -= code[j];
                let j = (i + 1 + k as usize) % n;
                sum += code[j];
            } else {
                sum += code[i];
                let j = ((i as i32 + k + n as i32) as usize) % n;
                sum -= code[j];
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
        assert_eq!(vec![12, 10, 16, 13], Solution::decrypt(vec![5, 7, 1, 4], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0, 0, 0, 0], Solution::decrypt(vec![1, 2, 3, 4], 0));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![12, 5, 6, 13], Solution::decrypt(vec![2, 4, 9, 3], -2));
    }
}
