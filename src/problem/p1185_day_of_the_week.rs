pub struct Solution {}

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let month_days = [
            31,
            28 + if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
                1
            } else {
                0
            },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        let weekday = [
            // 1970 年 12 月 31 日是星期四
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
            "Monday",
            "Tuesday",
        ];

        weekday[((day
            + month_day[..month as usize - 1].iter().sum::<i32>()
            + (1970..year).fold(0, |cnt, i| {
                cnt + 365
                    + if i % 400 == 0 || (i % 4 == 0 && i % 100 != 0) {
                        1
                    } else {
                        0
                    }
            }))
            % 7) as usize]
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1185() {
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
    }
}
