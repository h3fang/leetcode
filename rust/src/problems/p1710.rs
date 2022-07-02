pub struct Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|a| a[1]);
        let mut result = 0;
        for b in box_types.iter().rev() {
            let s = truck_size.min(b[0]);
            truck_size -= s;
            result += s * b[1];
            if truck_size == 0 {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let box_types = [[1, 3], [2, 2], [3, 1]];
        let box_types = box_types.iter().map(|b| b.to_vec()).collect();
        assert_eq!(8, Solution::maximum_units(box_types, 4));
    }
}
