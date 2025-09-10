use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::min_operations(vec![vec![1, 2], vec![2, 4]]), 3);
        assert_eq!(Solution::min_operations(vec![vec![2, 6]]), 4);
    }
}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        0
    }
}
