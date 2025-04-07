pub struct Solution {}

impl Solution {
    pub fn original_digits(s: String) -> String {
        let tra_info = [
            (0, b'z', "ero"),
            (2, b'w', "to"),
            (4, b'u', "for"),
            (6, b'x', "si"),
            (8, b'g', "eiht"),
            (1, b'o', "ne"),
            (3, b't', "hree"),
            (5, b'f', "ive"),
            (7, b's', "even"),
            (9, b'i', "nne"),
        ];

        // 统计所有字符的数量
        let mut letter_cnt = [0; 26];
        s.bytes().for_each(|b| letter_cnt[(b - b'a') as usize] += 1);

        let mut num_cnt = [0; 10];
        tra_info.iter().for_each(|info| {
            num_cnt[info.0] += letter_cnt[(info.1 - b'a') as usize];
            info.2.bytes().for_each(|b| {
                letter_cnt[(b - b'a') as usize] -= letter_cnt[(info.1 - b'a') as usize]
            });
        });

        let mut ans = Vec::with_capacity(num_cnt.iter().sum::<usize>());
        (0..10).for_each(|i| {
            if num_cnt[i] != 0 {
                ans.append(&mut vec![b'0' + i as u8; num_cnt[i] as usize]);
            }
        });

        String::from_utf8(ans).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_423() {
        assert_eq!(Solution::original_digits(String::from("owoztneoer")), "012");
        assert_eq!(Solution::original_digits(String::from("fviefuro")), "45");
    }
}
