use std::collections::{BTreeSet, HashMap};

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]),
            vec![-1, -1, 2, 1, -1, -1]
        );

        assert_eq!(
            Solution::avoid_flood(vec![69, 0, 0, 0, 69]),
            vec![-1, 69, 1, 1, -1]
        );
    }
}

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut ans = vec![-1; n];

        let mut full_days = HashMap::new(); // lake -> 是否已满
        let mut dry_days = BTreeSet::new(); // 晴天

        for (i, lake) in rains.into_iter().enumerate() {
            if lake == 0 {
                ans[i] = 1;
                dry_days.insert(i);
                continue;
            }

            if let Some(&j) = full_days.get(&lake) {
                if let Some(&d) = dry_days.range(j..).next() {
                    ans[d] = lake;
                    dry_days.remove(&d); // 移除已经被抽水的日子
                } else {
                    return vec![];
                }
            }

            full_days.insert(lake, i);
        }

        ans
    }
}
