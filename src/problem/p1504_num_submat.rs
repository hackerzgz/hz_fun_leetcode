use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            13
        );
        assert_eq!(
            Solution::num_submat(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]]),
            24
        );
    }
}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        0
    }
}
