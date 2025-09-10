use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::check_powers_of_three(12), true);
        assert_eq!(Solution::check_powers_of_three(91), true);
        assert_eq!(Solution::check_powers_of_three(21), false);
    }
}

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}
