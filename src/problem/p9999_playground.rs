use std::collections::HashMap;

use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(
            Solution::find_whether_exists_path(3, vec![vec![0, 1], vec![0, 2]], 0, 2),
            true
        );

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

        assert_eq!(
            Solution::find_longest_subarray(vec![
                "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I",
                "J", "K", "L", "M"
            ]),
            vec!["A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7"]
        );
    }
}

impl Solution {
    pub fn find_whether_exists_path(n: i32, graph: Vec<Vec<i32>>, start: i32, target: i32) -> bool {
        let n = n as usize;
        let start = start as usize;
        let target = target as usize;

        let mut next_graph = vec![vec![]; n];
        for edge in graph {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            next_graph[u].push(v);
        }

        let mut visited = vec![false; n];
        Self::fwep_dfs(&next_graph, &mut visited, start, target)
    }

    fn fwep_dfs(
        next_graph: &[Vec<usize>],
        visited: &mut Vec<bool>,
        curr: usize,
        target: usize,
    ) -> bool {
        if curr == target {
            return true;
        }
        visited[curr] = true;

        for &nb in &next_graph[curr] {
            if !visited[nb] && Self::fwep_dfs(next_graph, visited, nb, target) {
                return true;
            }
        }
        return false;
    }

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

    pub fn get_valid_t9_words(num: String, words: Vec<String>) -> Vec<String> {
        let tab = "22233344455566677778889999".as_bytes();
        let num = num.as_bytes();
        words
            .into_iter()
            .filter_map(|w| {
                let t: Vec<_> = w
                    .as_bytes()
                    .iter()
                    .map(|c| tab[(c - b'a') as usize])
                    .collect();
                if t == num {
                    Some(w)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut s = vec![0; array.len() + 1];
        for (i, c) in array.iter().enumerate() {
            s[i + 1] = s[i] + c.bytes().nth(0).unwrap() >> 6 & 1 * 2 - 1;

            // EQUAL
            // if c.bytes().nth(0).unwrap() >= b'A' && c.bytes().nth(0).unwrap() <= b'Z' {
            //     s[i + 1] = s[i] - 1;
            // } else {
            //     s[i + 1] = s[i] + 1;
            // }
        }

        let (mut begin, mut end) = (0, 0);
        let mut first = HashMap::new();
        for (i, prefix_sum) in s.iter().enumerate() {
            if let Some(&j) = first.get(&prefix_sum) {
                if i - j > begin - end {
                    begin = j;
                    end = i;
                }
            } else {
                first.insert(prefix_sum, i);
            }
        }
        return array[begin..end].to_vec();
    }
}
