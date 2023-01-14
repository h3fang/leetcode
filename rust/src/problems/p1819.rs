pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let mut f = vec![false; max as usize + 1];
        let mut result = 0;
        for n in nums {
            if !f[n as usize] {
                f[n as usize] = true;
                result += 1;
            }
        }
        for p in 1..max / 3 + 1 {
            if f[p as usize] {
                continue;
            }
            let mut g = 0;
            for i in (p * 2..=max).step_by(p as usize) {
                if f[i as usize] {
                    g = gcd(i, g);
                    if g == p {
                        break;
                    }
                }
            }
            if g == p {
                result += 1;
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
        assert_eq!(
            5,
            Solution::count_different_subsequence_gc_ds(vec![6, 10, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            7,
            Solution::count_different_subsequence_gc_ds(vec![5, 15, 40, 5, 6])
        );
    }
}
