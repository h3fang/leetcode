pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut ans = Vec::with_capacity(720.min(2 << turned_on));
        for h in 0..12u32 {
            let h1 = h.count_ones();
            for m in 0..60u32 {
                if m.count_ones() + h1 == turned_on {
                    ans.push(format!("{h}:{m:02}"));
                }
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
        let mut expected: Vec<String> = [
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        expected.sort_unstable();
        let mut result = Solution::read_binary_watch(1);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        assert!(Solution::read_binary_watch(9).is_empty());
    }
}
