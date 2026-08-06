pub struct Solution;

fn prod(mut n: i32) -> i32 {
    let mut p = 1;
    while n > 0 {
        p *= n % 10;
        n /= 10;
    }
    p
}

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for x in n.. {
            if prod(x) % t == 0 {
                return x;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::smallest_number(10, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::smallest_number(15, 3));
    }
}
