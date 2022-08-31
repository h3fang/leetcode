pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let blocks = blocks.as_bytes();
        let mut result = k;
        let mut white = 0;
        for (i, &c) in blocks.iter().enumerate() {
            if c == b'W' {
                white += 1;
            }
            if i >= k && blocks[i - k] == b'W' {
                white -= 1;
            }
            if i + 1 >= k {
                result = result.min(white);
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_recolors("WBWBBBW".to_string(), 2));
    }
}
