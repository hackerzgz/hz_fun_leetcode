pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(&self) {
        assert_eq!(Solution::divisor_game(2), true);
        assert_eq!(Solution::divisor_game(3), false);
    }

    pub fn divisor_game(n: i32) -> bool {
        if n & 1 == 0 {
            return true;
        }
        return false;
    }
}
