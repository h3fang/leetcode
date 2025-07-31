pub struct Solution;

struct XorBasis {
    b: Vec<u32>,
}

impl XorBasis {
    fn new(n: usize) -> Self {
        Self { b: vec![0; n] }
    }

    fn insert(&mut self, mut x: u32) {
        while x > 0 {
            let i = 32 - x.leading_zeros() as usize - 1;
            if self.b[i] == 0 {
                self.b[i] = x;
                return;
            }
            x ^= self.b[i];
        }
    }

    fn max_xor(&self) -> u32 {
        let mut ans = 0;
        for &b in self.b.iter().rev() {
            if ans ^ b > ans {
                ans ^= b;
            }
        }
        ans
    }
}

fn max_xor(sub: i32, nums: &[i32], m: usize, xor: &[i64]) -> i64 {
    let mut xb = XorBasis::new(m);
    let r = xor[sub as usize];
    for (i, &x) in nums.iter().enumerate() {
        if (sub >> i) & 1 == 1 {
            xb.insert((x as i64 & (!r)) as u32);
        }
    }
    r + xb.max_xor() as i64 * 2
}

impl Solution {
    pub fn maximize_xor_and_xor(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let u = 1 << n;

        let mut and = vec![0; u];
        let mut xor = vec![0; u];
        let mut or = vec![0; u];
        and[0] = -1;
        for (i, &x) in nums.iter().enumerate() {
            let high = 1 << i;
            let x = x as i64;
            for m in 0..high {
                and[high | m] = and[m] & x;
                xor[high | m] = xor[m] ^ x;
                or[high | m] = or[m] | x;
            }
        }
        and[0] = 0;

        let m = 64 - nums.iter().max().unwrap().leading_zeros() as usize;

        let mut ans = 0i64;
        for (i, &and) in and.iter().enumerate() {
            let j = (u - 1) ^ i;
            if (and + or[j] * 2 - xor[j]) > ans {
                ans = ans.max(and + max_xor(j as i32, &nums, m, &xor));
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
        assert_eq!(5, Solution::maximize_xor_and_xor(vec![2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::maximize_xor_and_xor(vec![1, 3, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(15, Solution::maximize_xor_and_xor(vec![2, 3, 6, 7]));
    }
}
