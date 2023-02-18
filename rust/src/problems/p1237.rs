pub struct Solution;

pub struct CustomFunction;
impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

impl Solution {
    pub fn find_solution(cf: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        fn binary(x: i32, z: i32, cf: &CustomFunction) -> Option<Vec<i32>> {
            let mut l = 1;
            let mut r = 1000;
            while l <= r {
                let y = (l + r) / 2;
                match cf.f(x, y).cmp(&z) {
                    std::cmp::Ordering::Less => l = y + 1,
                    std::cmp::Ordering::Equal => return Some(vec![x, y]),
                    std::cmp::Ordering::Greater => r = y - 1,
                }
            }
            None
        }
        (1..=1000).filter_map(|x| binary(x, z, cf)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [[1, 4], [2, 3], [3, 2], [4, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_solution(&CustomFunction, 5));
    }
}
