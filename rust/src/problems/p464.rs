pub struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        fn game(mask: u32, curr: i32, total: i32, max: i32, cache: &mut [u8]) -> bool {
            if cache[mask as usize] > 0 {
                return cache[mask as usize] == 1;
            }
            for x in 0..max {
                if mask & (1 << x) > 0 {
                    continue;
                }
                if curr + x + 1 >= total {
                    cache[mask as usize] = 1;
                    return true;
                }
                if !game(mask | (1 << x), curr + x + 1, total, max, cache) {
                    cache[mask as usize] = 1;
                    return true;
                }
            }
            cache[mask as usize] = 2;
            false
        }
        if max_choosable_integer >= desired_total {
            return true;
        }
        if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        let mut cache = vec![0; 2usize.pow(max_choosable_integer as u32) + 1];
        game(0, 0, desired_total, max_choosable_integer, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::can_i_win(10, 11));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_i_win(10, 0));
    }
}
