pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(&self) {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }

    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        piles[piles.len() / 3..].iter().step_by(2).sum()
    }
}
