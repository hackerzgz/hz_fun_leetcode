use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::reordered_power_of2(1), true);
        assert_eq!(Solution::reordered_power_of2(10), false);
    }
}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut nums = n.to_string().chars().collect::<Vec<char>>();
        nums.sort();

        for i in 0..32 {
            let mut tmp = 2_i32.pow(i).to_string().chars().collect::<Vec<char>>();
            tmp.sort();

            if tmp == nums {
                return true;
            }
        }
        return false;
    }
}
