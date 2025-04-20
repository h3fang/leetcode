pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let mut ans = vec![0; k];
        let mut prev = [0; 5];
        for x in nums {
            let x = (x as usize) % k;
            let mut next = [0; 5];
            next[x] = 1;
            for i in 0..k {
                next[(i * x) % k] += prev[i];
            }
            for (i, e) in ans.iter_mut().enumerate() {
                *e += next[i];
            }
            prev = next;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![9, 2, 4],
            Solution::result_array(vec![1, 2, 3, 4, 5], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![18, 1, 2, 0],
            Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(vec![9, 6], Solution::result_array(vec![1, 1, 2, 1, 1], 2));
    }
}
