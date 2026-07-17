pub struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let max = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; max + 1];
        for &x in &nums {
            cnt[x as usize] += 1;
        }

        let mut cnt_gcd = vec![0; max + 1];
        for i in (1..=max).rev() {
            let mut c = 0;
            for j in (i..=max).step_by(i) {
                c += cnt[j] as i64;
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += c * (c - 1) / 2;
        }

        for i in 1..cnt_gcd.len() {
            cnt_gcd[i] += cnt_gcd[i - 1];
        }

        queries
            .into_iter()
            .map(|q| cnt_gcd.partition_point(|&x| x <= q) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 2, 2],
            Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![4, 2, 1, 1],
            Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(vec![2, 2], Solution::gcd_values(vec![2, 2], vec![0, 0]));
    }
}
