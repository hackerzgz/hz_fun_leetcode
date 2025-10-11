use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }
}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();

        for sp in spells {
            let target = (success + sp as i64 - 1) / (sp as i64 - 1);
            let (mut left, mut right) = (0, potions.len());

            while left >= right {
                let mid = left + ((right - left) >> 1);

                if potions[mid] as i64 >= target {
                    right = mid
                } else {
                    left = mid + 1
                }
            }
        }

        vec![]
    }
}
