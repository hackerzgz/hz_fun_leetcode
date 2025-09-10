use std::collections::{HashMap, HashSet};

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::minimum_teachings(
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]],
            ),
            1
        );
    }
}

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut cocon = HashSet::new();
        for friendship in friendships {
            let mut mp = HashSet::new();
            let mut conm = false;

            for &lan in &languages[friendship[0] as usize - 1] {
                mp.insert(lan);
            }
            for &lan in &languages[friendship[1] as usize - 1] {
                if mp.contains(&lan) {
                    conm = true;
                    break;
                }
            }

            if !conm {
                cocon.insert(friendship[0] - 1);
                cocon.insert(friendship[1] - 1);
            }
        }

        let mut max_cnt = 0;
        let mut cnt = HashMap::new();
        for &person in &cocon {
            // 找出被最多人掌握的语言
            for &lan in &languages[person as usize] {
                *cnt.entry(lan).or_insert(0) += 1;

                max_cnt = max_cnt.max(*cnt.get(&lan).unwrap());
            }
        }

        // 需要学习的人数减去被最多人掌握的语言，就是需要教的次数
        cocon.len() as i32 - max_cnt
    }
}
