use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let masks: Vec<i32> = words
            .iter()
            .map(|word| {
                word.chars()
                    .fold(0, |acc, c| acc | 1 << (c as u8 - 'a' as u8))
            })
            .collect();

        let mut ans = 0;
        for i in 0..masks.len() {
            for j in i + 1..masks.len() {
                if masks[i] & masks[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }
        ans as i32
    }

    pub fn max_product_fast(words: Vec<String>) -> i32 {
        let masks = words.iter().fold(HashMap::new(), |mut map, w| {
            let mask = w.chars().fold(0, |acc, c| acc | 1 << (c as u8 - 'a' as u8));

            let length = w.len();
            match map.get(&mask) {
                Some(value) if &length > value => {
                    map.insert(mask, length);
                }
                None => {
                    map.insert(mask, length);
                }
                _ => {}
            };
            map
        });

        let mut ans = 0;
        for (k1, l1) in masks.iter() {
            for (k2, l2) in masks.iter() {
                if (k1 & k2 == 0) {
                    ans = ans.max(l1 * l2);
                }
            }
        }
        ans as i32
    }
}
