pub struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let bulky = (length >= 10000 || width >= 10000 || height >= 10000)
            || length as u64 * width as u64 * height as u64 >= 10_0000_0000;
        let heavy = mass >= 100;
        if bulky && heavy {
            "Both".into()
        } else if !bulky && !heavy {
            "Neither".into()
        } else if bulky {
            "Bulky".into()
        } else {
            "Heavy".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("Heavy", Solution::categorize_box(1000, 35, 700, 300));
    }

    #[test]
    fn case2() {
        assert_eq!("Neither", Solution::categorize_box(200, 50, 800, 50));
    }
}
