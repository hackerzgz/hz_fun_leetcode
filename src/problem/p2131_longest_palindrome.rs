use std::{cmp::min, collections::HashMap};

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        Solution {}
    }

    pub fn run(&self) {
        assert_eq!(
            Solution::longest_palindrome(vec![
                "lc".to_string(),
                "cl".to_string(),
                "gg".to_string()
            ]),
            6
        );

        assert_eq!(
            Solution::longest_palindrome_fast(vec![
                "lc".to_string(),
                "cl".to_string(),
                "gg".to_string()
            ]),
            6
        );
    }

    fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut freq = HashMap::new();
        for word in &words {
            *freq.entry(word.clone()).or_insert(0) += 1;
        }

        let mut res = 0;
        let mut mid = false;
        for (word, count) in &freq {
            let rev = format!(
                "{}{}",
                word.chars().nth(1).unwrap(),
                word.chars().nth(0).unwrap(),
            );

            if *word == rev {
                if count % 2 == 1 {
                    mid = true
                }
                res += 2 * (count / 2 * 2);
            } else if *word > rev {
                res += 4 * min(*count, *freq.get(&rev).unwrap_or(&0));
            }
        }
        if mid {
            res += 2;
        }
        res
    }

    fn longest_palindrome_fast(words: Vec<String>) -> i32 {
        let mut cnt = [[0; 26]; 26];
        for w in words {
            let w = w.into_bytes();
            cnt[(w[0] - b'a') as usize][(w[1] - b'a') as usize] += 1;
        }

        let mut ans = 0;
        let mut odd = 0;
        for i in 0..26 {
            let c = cnt[i][i];
            ans += c - c % 2; // 保证结果是偶数，也可以写成 c & !1
            odd |= c % 2; // 存在出现奇数次的 cnt[i][i]
            for j in i + 1..26 {
                ans += cnt[i][j].min(cnt[j][i]) * 2;
            }
        }
        (ans + odd) * 2 // 上面统计的是字符串个数，乘以 2 就是长度
    }
}
