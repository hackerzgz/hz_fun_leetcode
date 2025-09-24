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
                "A".to_string(),
                "1".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "E".to_string(),
                "5".to_string(),
                "F".to_string(),
                "G".to_string(),
                "6".to_string(),
                "7".to_string(),
                "H".to_string(),
                "I".to_string(),
                "J".to_string(),
                "K".to_string(),
                "L".to_string(),
                "M".to_string()
            ]),
            vec!["A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7"]
        );

        assert_eq!(
            Solution::pair_sums(vec![5, 6, 5, 6], 11),
            vec![vec![5, 6], vec![5, 6]]
        );

        assert_eq!(
            Solution::truly_most_popular(
                vec![
                    "John(15)".to_string(),
                    "Jon(12)".to_string(),
                    "Chris(13)".to_string(),
                    "Kris(4)".to_string(),
                    "Christopher(19)".to_string(),
                ],
                vec![
                    "(Jon,John)".to_string(),
                    "(John,Johnny)".to_string(),
                    "(Chris,Kris)".to_string(),
                    "(Chris,Christopher)".to_string(),
                ],
            ),
            vec!["John(27)", "Chris(36)"]
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
            s[i + 1] = s[i] + ((c.bytes().nth(0).unwrap() >> 6 & 1) as i8 * 2 - 1);

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
                if i - j > end - begin {
                    begin = j;
                    end = i;
                    first.insert(prefix_sum, j);
                }
            } else {
                first.insert(prefix_sum, i);
            }
        }
        return array[begin..end].to_vec();
    }

    pub fn pair_sums(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 2 {
            return vec![];
        }

        nums.sort();

        let (mut left, mut right) = (0, nums.len() - 1);
        let mut ans = Vec::with_capacity(4);

        while left < right {
            let lhs = nums[left];
            let rhs = nums[right];
            if lhs + rhs == target {
                ans.push(vec![lhs, rhs]);
                left += 1;
                right -= 1;
            } else if lhs + rhs < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        ans
    }

    pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
        let mut counts: HashMap<String, i32> = HashMap::new();
        for name in names {
            let left = name.find('(').unwrap();
            // println!("left: {}", left);
            let count = name[left + 1..name.len() - 1].parse::<i32>().unwrap();
            let raw_name = name[0..left].to_string();
            counts.entry(raw_name).or_insert(count);
        }

        // 并查集
        // <node, root>
        let mut mp: HashMap<String, String> = HashMap::new();
        for synonym in synonyms {
            let comm = synonym.find(',').unwrap();
            let mut left = synonym[1..comm].to_string();
            let mut right = synonym[comm + 1..synonym.len() - 1].to_string();

            while mp.contains_key(&left) {
                left = mp.get(&left).unwrap().clone();
            }
            while mp.contains_key(&right) {
                right = mp.get(&right).unwrap().clone();
            }
            // 如果是共同的祖先，那么就不用再处理了
            if left == right {
                continue;
            }

            let left_count = *counts.get(&left).unwrap_or(&0);
            let right_count = *counts.get(&right).unwrap_or(&0);

            // 找到新根节点以及当前节点
            let root = left.clone().min(right.clone());
            let node = left.clone().max(right.clone());

            while !counts.contains_key(&left) && !counts.contains_key(&right) {
                mp.insert(node.clone(), root.clone());
                continue;
            }

            // 移除当前节点总数
            counts.remove(node.as_str());
            // 将当前节点总数累加到根节点上
            counts.insert(root.clone(), left_count + right_count);
            // 添加映射关系
            mp.insert(node, root);
        }

        counts
            .iter()
            .map(|(name, count)| format!("{}({})", name, count))
            .collect()
    }
}
