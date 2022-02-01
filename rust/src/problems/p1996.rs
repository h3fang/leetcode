pub struct Solution;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable();
        let mut result = 0;
        let mut lst_a = i32::MAX;
        let mut max_d = i32::MIN;
        let mut cur = 0;
        for p in properties.into_iter().rev() {
            let (a, d) = (p[0], p[1]);
            if a != lst_a {
                cur = max_d
            }
            result += (d < cur) as i32;
            max_d = max_d.max(d);
            lst_a = a;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let properties = [[5, 5], [6, 3], [3, 6]];
        let properties = properties.iter().map(|p| p.to_vec()).collect();
        assert_eq!(0, Solution::number_of_weak_characters(properties));
    }

    #[test]
    fn case2() {
        let properties = [[2, 2], [3, 3]];
        let properties = properties.iter().map(|p| p.to_vec()).collect();
        assert_eq!(1, Solution::number_of_weak_characters(properties));
    }

    #[test]
    fn case3() {
        let properties = [[1, 5], [10, 4], [4, 3]];
        let properties = properties.iter().map(|p| p.to_vec()).collect();
        assert_eq!(1, Solution::number_of_weak_characters(properties));
    }
}
