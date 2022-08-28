pub struct Solution;

impl Solution {
    pub fn min_number_of_hours(
        mut initial_energy: i32,
        mut initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut result = 0;
        for (&e, &exp) in energy.iter().zip(&experience) {
            if initial_energy <= e {
                result += e - initial_energy + 1;
                initial_energy = 1;
            } else {
                initial_energy -= e;
            }

            if initial_experience <= exp {
                result += exp - initial_experience + 1;
                initial_experience = exp + 1;
            }
            initial_experience += exp;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            8,
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1])
        );
    }
}
