pub struct Solution;

fn update(b: u8, seg: &mut i32, mask: &mut i32, size: &mut i32, k: i32) {
    let bit = 1 << (b - b'a');
    if *mask & bit > 0 {
        return;
    }
    *size += 1;
    if *size > k {
        *seg += 1;
        *mask = bit;
        *size = 1;
    } else {
        *mask |= bit;
    }
}

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        if k == 26 {
            return 1;
        }

        let s = s.as_bytes();
        let n = s.len();

        let (mut seg, mut mask, mut size) = (1, 0, 0);
        let mut suf = vec![(0, 0); n + 1];

        for (i, &b) in s.iter().enumerate().rev() {
            update(b, &mut seg, &mut mask, &mut size, k);
            suf[i] = (seg, mask);
        }

        let mut ans = seg;
        (seg, mask, size) = (1, 0, 0);
        for (i, (suf_seg, suf_mask)) in suf.into_iter().enumerate().skip(1) {
            let mut r = seg + suf_seg;
            let bits = (mask | suf_mask).count_ones() as i32;
            if bits < k {
                r -= 1;
            } else if bits < 26 && size == k && suf_mask.count_ones() as i32 == k {
                r += 1;
            }
            ans = ans.max(r);
            update(s[i - 1], &mut seg, &mut mask, &mut size, k);
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
            3,
            Solution::max_partitions_after_operations("accca".to_string(), 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::max_partitions_after_operations("aabaab".to_string(), 3)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            4,
            Solution::max_partitions_after_operations("xxyz".to_string(), 1)
        );
    }
}
