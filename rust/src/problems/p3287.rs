pub struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        fn find_ors<'a>(nums: impl Iterator<Item = &'a i32>, k: i32) -> Vec<[bool; 128]> {
            let mut prev = vec![[false; 128]; k as usize + 1];
            prev[0][0] = true;

            nums.enumerate()
                .map(|(i, &x)| {
                    for j in (0..=(k as usize - 1).min(i + 1)).rev() {
                        let (before, after) = prev.split_at_mut(j + 1);
                        for (y, &v) in before[j].iter().enumerate() {
                            if v {
                                after[0][y | x as usize] = true;
                            }
                        }
                    }
                    prev[k as usize]
                })
                .collect()
        }

        let a = find_ors(nums.iter(), k);
        let b = find_ors(nums.iter().rev(), k);
        let mut mx = 0;
        for i in k as usize - 1..nums.len() - k as usize {
            for (a, &va) in a[i].iter().enumerate() {
                if !va {
                    continue;
                }
                for (b, &vb) in b[nums.len() - i - 2].iter().enumerate() {
                    if vb {
                        mx = mx.max(a ^ b);
                    }
                }
            }
        }
        mx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_value(vec![2, 6, 7], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_value(vec![4, 2, 5, 6, 7], 2));
    }
}
