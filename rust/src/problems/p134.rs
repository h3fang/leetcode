pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut fuel = 0;
        let mut min_fuel = i32::MAX;
        let mut min = gas.len();
        for (i, (g, c)) in gas.iter().zip(&cost).enumerate() {
            fuel += g - c;
            if fuel < min_fuel {
                min_fuel = fuel;
                min = i;
            }
        }
        if fuel >= 0 {
            ((min + 1) % gas.len()) as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(3, Solution::can_complete_circuit(gas, cost));
    }

    #[test]
    fn case2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(-1, Solution::can_complete_circuit(gas, cost));
    }
}
