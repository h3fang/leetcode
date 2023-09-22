pub struct Solution;

impl Solution {
    pub fn dist_money(mut money: i32, mut children: i32) -> i32 {
        if money < children {
            return -1;
        }
        money -= children;
        let mut r = (money / 7).min(children);
        money -= r * 7;
        children -= r;
        if (children == 0 && money > 0) || (children == 1 && money == 3) {
            r -= 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::dist_money(20, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::dist_money(16, 2));
    }
}
