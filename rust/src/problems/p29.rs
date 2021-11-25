pub struct Solution;

impl Solution {
    pub fn divide(mut a: i32, mut b: i32) -> i32 {
        // check whether b * c > a
        fn quick_multiply(a: i32, b: i32, mut c: i32) -> bool {
            let mut result = 0;
            let mut contrib = b;
            while c > 0 {
                if (c & 1) == 1 {
                    if result < a - contrib {
                        return false;
                    }
                    result += contrib;
                }
                if c > 1 {
                    #[allow(clippy::overflow_check_conditional)]
                    if contrib < a - contrib {
                        return false;
                    }
                    contrib += contrib;
                }

                c >>= 1;
            }
            true
        }
        if a == i32::MIN {
            if b == 1 {
                return i32::MIN;
            } else if b == -1 {
                return i32::MAX;
            }
        }

        if b == i32::MIN {
            if a == i32::MIN {
                return 1;
            } else {
                return 0;
            }
        }

        let negative = (a < 0) ^ (b < 0);
        if a > 0 {
            a = -a;
        }
        if b > 0 {
            b = -b;
        }
        let mut left = 1;
        let mut right = i32::MAX;
        let mut result = 0;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            if quick_multiply(a, b, mid) {
                result = mid;
                if mid == i32::MAX {
                    break;
                }
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        if negative {
            -result
        } else {
            result
        }
    }

    pub fn divide_v2(mut a: i32, mut b: i32) -> i32 {
        fn div(a: i32, b: i32) -> i32 {
            if a > b {
                return 0;
            }
            let mut r = 1;
            let mut m = b;
            while m >= a - m {
                r += r;
                m += m;
            }
            r + div(a - m, b)
        }
        if b == 1 {
            return a;
        } else if b == -1 {
            if a == i32::MIN {
                return i32::MAX;
            } else {
                return -a;
            }
        }

        let negative = (a < 0) ^ (b < 0);
        if a > 0 {
            a = -a;
        }
        if b > 0 {
            b = -b;
        }
        let r = div(a, b);
        if negative {
            -r
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15 / 2, Solution::divide(15, 2));
        assert_eq!(15 / 2, Solution::divide_v2(15, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(7 / -3, Solution::divide(7, -3));
        assert_eq!(7 / -3, Solution::divide_v2(7, -3));
    }

    #[test]
    fn case3() {
        assert_eq!(i32::MIN / 7, Solution::divide(i32::MIN, 7));
        assert_eq!(i32::MIN / 7, Solution::divide_v2(i32::MIN, 7));
    }

    #[test]
    fn case4() {
        assert_eq!(i32::MIN / i32::MAX, Solution::divide(i32::MIN, i32::MAX));
        assert_eq!(i32::MIN / i32::MAX, Solution::divide_v2(i32::MIN, i32::MAX));
    }
}
