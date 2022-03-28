pub struct Solution;

impl Solution {
    pub fn min_cost_set_time(
        start_at: i32,
        move_cost: i32,
        push_cost: i32,
        target_seconds: i32,
    ) -> i32 {
        fn cost(m: i32, s: i32, mut start_at: i32, move_cost: i32, push_cost: i32) -> i32 {
            let m1 = m / 10;
            let m2 = m % 10;
            let s1 = s / 10;
            let s2 = s % 10;
            let mut result = 0;
            result += if m1 == 0 {
                0
            } else {
                let r = if m1 == start_at {
                    push_cost
                } else {
                    move_cost + push_cost
                };
                start_at = m1;
                r
            };

            result += if m1 == 0 && m2 == 0 {
                0
            } else {
                let r = if m2 == start_at {
                    push_cost
                } else {
                    move_cost + push_cost
                };
                start_at = m2;
                r
            };

            result += if m1 == 0 && m2 == 0 && s1 == 0 {
                0
            } else {
                let r = if s1 == start_at {
                    push_cost
                } else {
                    move_cost + push_cost
                };
                start_at = s1;
                r
            };

            result += if m1 == 0 && m2 == 0 && s1 == 0 && s2 == 0 {
                0
            } else if s2 == start_at {
                push_cost
            } else {
                move_cost + push_cost
            };

            result
        }

        let m = target_seconds / 60;
        let s = target_seconds - m * 60;
        let r1 = if m > 99 {
            i32::MAX
        } else {
            cost(m, s, start_at, move_cost, push_cost)
        };
        if s <= 39 && m >= 1 {
            r1.min(cost(m - 1, s + 60, start_at, move_cost, push_cost))
        } else {
            r1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::min_cost_set_time(1, 2, 1, 600));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::min_cost_set_time(0, 1, 2, 76));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::min_cost_set_time(1, 1, 1, 1));
    }
}
