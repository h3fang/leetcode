pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let max = *arr.iter().max().unwrap();
        let mut presum = vec![0; max as usize + 2];
        let mut ans = 0;
        for (j, &y) in arr.iter().enumerate().skip(1) {
            for x in arr[j - 1] + 1..max + 2 {
                presum[x as usize] += 1;
            }
            for &z in arr.iter().skip(j + 1) {
                if (y - z).abs() > b {
                    continue;
                }
                let lb = 0.max(y - a).max(z - c);
                let ub = max.min(y + a).min(z + c);
                ans += (presum[ub as usize + 1] - presum[lb as usize]).max(0);
            }
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
            4,
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1)
        );
    }
}
