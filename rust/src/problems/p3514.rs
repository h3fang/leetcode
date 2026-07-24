pub struct Solution;

fn fwt_xor(a: &mut [i64], shift: u32) {
    let n = a.len();

    let (mut l, mut k) = (2, 1);
    while l <= n {
        for i in (0..n).step_by(l) {
            for j in 0..k {
                (a[i + j], a[i + j + k]) = (
                    (a[i + j] + a[i + j + k]) >> shift,
                    (a[i + j] - a[i + j + k]) >> shift,
                );
            }
        }

        (l, k) = (l << 1, k << 1);
    }
}

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let max = 1usize << (i32::BITS - max.leading_zeros());

        let mut cnt = vec![0; max];
        for x in nums {
            cnt[x as usize] += 1;
        }

        fwt_xor(&mut cnt, 0);
        cnt.iter_mut().for_each(|e| *e = *e * *e * *e);
        fwt_xor(&mut cnt, 1);

        cnt.into_iter().filter(|&e| e > 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::unique_xor_triplets(vec![1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::unique_xor_triplets(vec![6, 7, 8, 9]));
    }
}
