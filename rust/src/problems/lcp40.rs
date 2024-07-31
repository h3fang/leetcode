pub struct Solution;

impl Solution {
    pub fn maxmium_score(mut cards: Vec<i32>, cnt: i32) -> i32 {
        let (n, cnt) = (cards.len(), cnt as usize);
        cards.sort_unstable();
        let (mut result, mut odd, mut odd_min, mut even_min) = (0, 0, i32::MAX, i32::MAX);
        for &x in cards.iter().rev().take(cnt) {
            result += x;
            if x % 2 == 1 {
                odd += 1;
                odd_min = x;
            } else {
                even_min = x;
            }
        }
        if odd % 2 == 1 {
            let a = result - odd_min
                + cards[..n - cnt]
                    .iter()
                    .rfind(|&&x| x % 2 == 0)
                    .unwrap_or(&i32::MIN);
            let b = if even_min < i32::MAX {
                result - even_min
                    + cards[..n - cnt]
                        .iter()
                        .rfind(|&&x| x % 2 == 1)
                        .unwrap_or(&i32::MIN)
            } else {
                0
            };
            a.max(b).max(0)
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(18, Solution::maxmium_score(vec![1, 2, 8, 9], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::maxmium_score(vec![3, 3, 1], 1));
    }
}
