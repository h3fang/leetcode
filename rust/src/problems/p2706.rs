pub struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let (mut p1, mut p2) = (i32::MAX, i32::MAX);
        for p in prices {
            if p < p1 {
                (p1, p2) = (p, p1);
            } else if p < p2 {
                p2 = p;
            }
        }
        if p1 + p2 > money {
            money
        } else {
            money - (p1 + p2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::buy_choco(vec![1, 2, 2], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::buy_choco(vec![3, 2, 3], 3));
    }
}
