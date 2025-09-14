use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::does_alice_win("leetcoder".to_string()), true);
        assert_eq!(Solution::does_alice_win("bbcd".to_string()), false);
    }
}

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.bytes().any(|c| "aeiou".contains(c as char))
    }
}
