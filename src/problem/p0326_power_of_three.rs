use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::is_power_of_three(27), true);
        assert_eq!(Solution::is_power_of_three(0), false);
        assert_eq!(Solution::is_power_of_three(9), true);
        assert_eq!(Solution::is_power_of_three(45), false);
    }
}

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n > 0 && n % 3 == 0 {
            n /= 3;
        }

        return n == 1;
    }
}
