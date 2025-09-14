use crate::problem::Solver;

pub struct Solution {}

impl Solver for Solution {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        assert_eq!(Solution::sort_vowels("lEetcOde".to_string()), "lEOtcede");
        assert_eq!(Solution::sort_vowels("lYmpH".to_string()), "lYmpH");
    }
}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        const VOWEL_MASK: u32 = 0x208222;
        let mut cnt = [0; 'z' as usize + 1];
        for b in s.bytes() {
            if (VOWEL_MASK >> (b & 31)) & 1 > 0 {
                cnt[b as usize] += 1;
            }
        }

        let mut s = s.into_bytes();
        let mut j = 0u8;
        for b in s.iter_mut() {
            if (VOWEL_MASK >> (*b & 31)) & 1 == 0 {
                continue;
            }

            while cnt[j as usize] == 0 {
                if j == b'Z' {
                    j = b'a';
                } else {
                    j += 1;
                }
            }

            *b = j;
            cnt[j as usize] -= 1;
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}
