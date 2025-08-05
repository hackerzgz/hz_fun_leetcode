use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }

    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut cnts = HashMap::new();
        let mut left = 0;

        let mut ans = 0i32;
        for (right, curr_num) in fruits.iter().enumerate() {
            cnts.entry(curr_num).and_modify(|c| *c += 1).or_insert(1);

            while cnts.len() > 2 {
                let f = &fruits[left];
                cnts.entry(f).and_modify(|c| *c -= 1);
                if cnts[f] == 0 {
                    cnts.remove(f);
                }
                left += 1;
            }

            ans = ans.max((right - left + 1) as i32);
        }

        ans
    }
}
