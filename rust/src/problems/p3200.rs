pub struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn f(odd: i32, even: i32) -> i32 {
            let (mut o, mut e) = (0, 0);
            for i in 1.. {
                if i % 2 == 1 {
                    o += i;
                } else {
                    e += i;
                }
                if o > odd || e > even {
                    return i - 1;
                }
            }
            unreachable!()
        }
        f(red, blue).max(f(blue, red))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_height_of_triangle(2, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_height_of_triangle(2, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::max_height_of_triangle(1, 1));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::max_height_of_triangle(10, 1));
    }
}
