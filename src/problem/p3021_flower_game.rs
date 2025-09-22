use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::flower_game(3, 2), 3);
    }
}

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        n as i64 * m as i64 / 2
    }
}
