pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by_key(|p| (p[0], -p[1]));
        let mut result = vec![vec![0; 0]; people.len()];
        for p in people {
            let mut i = p[1] + 1;
            for e in result.iter_mut() {
                if e.is_empty() {
                    i -= 1;
                    if i == 0 {
                        *e = p;
                        break;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_array(arr: &[[i32; 2]]) -> Vec<Vec<i32>> {
        arr.iter().map(|a| a.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let people = parse_array(&[[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]);
        let expected = parse_array(&[[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]);
        let result = Solution::reconstruct_queue(people);
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let people = parse_array(&[[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]);
        let expected = parse_array(&[[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]]);
        let result = Solution::reconstruct_queue(people);
        assert_eq!(expected, result);
    }
}
