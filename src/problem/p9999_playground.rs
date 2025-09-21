use std::num;

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Self::diving_board(1, 2, 3), vec![3, 4, 5, 6]);

        assert_eq!(
            Solution::master_mind("RGRB".to_string(), "BBBY".to_string()),
            vec![0, 1]
        );

        assert_eq!(
            Solution::sub_sort(vec![1, 2, 4, 7, 10, 11, 7, 12, 6, 7, 16, 18, 19]),
            vec![3, 9]
        );

        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}

impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        if shorter == longer {
            return vec![shorter * k];
        }

        let base = shorter * k;
        let diff = longer - shorter;
        (0..=k).map(|i| base + (diff * i)).collect()
    }

    pub fn master_mind(solution: String, guess: String) -> Vec<i32> {
        // 猜中个数
        let mut r = 0;
        // 伪猜中个数
        let mut w = 0;
        // 因为只有 R，Y，G，B四种颜色，用数组记录每个颜色出现的次数
        let mut solution_rygb = [0; 4];
        let mut guess_rygb = [0; 4];

        fn color_index(color: u8) -> usize {
            match color {
                b'R' => 0,
                b'Y' => 1,
                b'G' => 2,
                b'B' => 3,
                _ => panic!("invalid color"),
            }
        }

        let gb = guess.as_bytes();
        let sb = solution.as_bytes();
        for i in 0..solution.len() {
            if sb[i] == gb[i] {
                r += 1;
            } else {
                solution_rygb[color_index(sb[i])] += 1;
                guess_rygb[color_index(gb[i])] += 1;
            }
        }

        for i in 0..4 {
            w += solution_rygb[i].min(guess_rygb[i]);
        }
        vec![r, w]
    }

    pub fn sub_sort(array: Vec<i32>) -> Vec<i32> {
        let mut last = -1;
        let mut first = -1;
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        let array_len = array.len();
        for i in 0..array.len() {
            if array[i] < max {
                last = i as i32;
            } else {
                max = array[i];
            }

            if array[array_len - 1 - i] > min {
                first = array_len as i32 - i as i32 - 1;
            } else {
                min = array[array_len - 1 - i];
            }
        }

        vec![first, last]
    }

    // DP
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut pre = 0i32;
        let mut ans = nums[0];

        for n in nums {
            pre = n.max(pre + n);
            ans = ans.max(pre);
        }

        ans
    }
}
