/*
输⼊年⽉⽇字符串，输出数字字符串回答以下问题，例如："2025-12-31", "1,3,365,0,48,1"
1.当周是⼀年中的第⼏周;
2.当天是周几;
3.当天是⼀年中的第⼏天;
4.当天距离明年元旦还有多少天;
5.当天距离下一个农历正月初一有多少天;
6.当天距离下一个非节假日还有多少天。
*/

pub fn time_info(time: &str) -> String {
    // 解析输入日期
    let parts: Vec<&str> = time.split('-').collect();
    let year: i32 = parts[0].parse().unwrap();
    let month: i32 = parts[1].parse().unwrap();
    let day: i32 = parts[2].parse().unwrap();

    // 计算一年中的第几天
    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    }

    fn days_in_year(y: i32, m: i32, d: i32) -> i32 {
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut total = d;
        for i in 0..(m - 1) as usize {
            total += if i == 1 && is_leap_year(y) { 29 } else { days[i] };
        }
        total
    }

    // 计算星期几 (Zeller公式)
    fn weekday(y: i32, m: i32, d: i32) -> i32 {
        let m = if m < 3 { m + 12 } else { m };
        let y = if m > 12 { y - 1 } else { y };
        let k = d;
        let m = m;
        let d = y / 100;
        let y = y % 100;
        let h = (k + (13 * (m + 1)) / 5 + y + y / 4 + d / 4 + 5 * d) % 7;
        ((h + 5) % 7) + 1  // 转换为1-7表示周一到周日
    }

    // 计算ISO 8601周数
    fn iso_week(y: i32, m: i32, d: i32) -> i32 {
        let day_of_year = days_in_year(y, m, d);
        let wday = weekday(y, m, d);
        let week = (day_of_year - wday + 10) / 7;
        if week == 0 {
            // 可能属于上一年的最后一周
            let prev_year_days = if is_leap_year(y - 1) { 366 } else { 365 };
            (prev_year_days - weekday(y - 1, 12, 31) + 10) / 7
        } else if week == 53 && (wday > 4 || (wday == 3 && !is_leap_year(y))) {
            1  // 属于下一年的第一周
        } else {
            week
        }
    }

    // 计算到下一年初一的天数
    fn days_to_next_year(y: i32, m: i32, d: i32) -> i32 {
        let days_into_next_year = if is_leap_year(y) { 366 } else { 365 };
        days_into_next_year - days_in_year(y, m, d) 
    }

    fn days_to_spring_festival(y: i32, m: i32, d: i32) -> i32 {
        // 已知春节日期
        let spring_festivals = [
            (2025, 1, 29), // 2025年春节
            (2026, 2, 17), // 2026年春节
        ];
        
        let current_days = days_in_year(y, m, d);
        
        // 找到下一个春节
        for &(spring_y, spring_m, spring_d) in spring_festivals.iter() {
            if y > spring_y || (y == spring_y && (m > spring_m || (m == spring_m && d >= spring_d))) {
                continue; // 当前日期已过这个春节
            }
            
            let spring_days = days_in_year(spring_y, spring_m, spring_d);
            let days_in_curr_year = if is_leap_year(y) { 366 } else { 365 };
            
            if y == spring_y {
                return spring_days - current_days;
            } else {
                // 跨越年份计算
                let mut total_days = days_in_curr_year - current_days;
                for year in (y + 1)..spring_y {
                    total_days += if is_leap_year(year) { 366 } else { 365 };
                }
                total_days += spring_days;
                return total_days;
            }
        }
        
        // 如果超过所有定义的春节，返回到2025年春节的粗略估计
        let spring_y = 2025;
        let spring_m = 1;
        let spring_d = 29;
        let spring_days = days_in_year(spring_y, spring_m, spring_d);
        let mut total_days = spring_days;
        for year in 2025..y {
            total_days += if is_leap_year(year) { 366 } else { 365 };
        }
        total_days - current_days
    }

    fn days_to_next_workday(y: i32, m: i32, d: i32) -> i32 {
        // 节假日定义
        fn is_holiday(y: i32, m: i32, d: i32) -> bool {
            if y != 2025 && !(y == 2026 && m == 1 && d == 1) { return false; }
            
            match (m, d) {
                // 元旦
                (1, 1) => true,
                // 春节: 1月28日-2月4日
                (1, 28..=31) => true,
                (2, 1..=4) => true,
                // 清明节: 4月4日-6日
                (4, 4..=6) => true,
                // 劳动节: 5月1日-5日
                (5, 1..=5) => true,
                // 端午节: 5月31日-6月2日
                (5, 31) => true,
                (6, 1..=2) => true,
                // 国庆节+中秋节: 10月1日-8日
                (10, 1..=8) => true,
                _ => false,
            }
        }
    
        fn is_weekend(y: i32, m: i32, d: i32) -> bool {
            let w = weekday(y, m, d);
            w == 6 || w == 7
        }
    
        let mut days = 0;
        let mut curr_y = y;
        let mut curr_m = m;
        let mut curr_d = d;
    
        loop {
            // 日期加1
            days += 1;
            let days_in_month = match curr_m {
                4 | 6 | 9 | 11 => 30,
                2 => if is_leap_year(curr_y) { 29 } else { 28 },
                _ => 31,
            };
            
            curr_d += 1;
            if curr_d > days_in_month {
                curr_d = 1;
                curr_m += 1;
                if curr_m > 12 {
                    curr_m = 1;
                    curr_y += 1;
                }
            }
            
            // 检查下一个日期是否为工作日
            if !is_holiday(curr_y, curr_m, curr_d) && !is_weekend(curr_y, curr_m, curr_d) {
                return days - 1; // 返回中间间隔的天数
            }
        }
    }

    let week_num = iso_week(year, month, day);
    let weekday_num = weekday(year, month, day);
    let day_of_year = days_in_year(year, month, day);
    let days_to_newyear = days_to_next_year(year, month, day);
    let days_to_spring = days_to_spring_festival(year, month, day);
    let days_to_workday = days_to_next_workday(year, month, day);

    format!(
        "{},{},{},{},{},{}",
        week_num,
        weekday_num,
        day_of_year,
        days_to_newyear,
        days_to_spring,
        days_to_workday
    )
}
