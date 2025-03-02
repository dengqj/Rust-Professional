// 日期结构体
struct Date {
    year: i32,
    month: i32,
}



// 人员类型枚举
enum PersonType {
    Male,           // 男职工
    Female55,       // 原55岁退休女职工
    Female50,       // 原50岁退休女职工
}

pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生日期
    let parts: Vec<&str> = time.split('-').collect();
    let birth_date = Date {
        year: parts[0].parse().unwrap(),
        month: parts[1].parse().unwrap(),
    };
    
    // 解析人员类型
    let person_type = match tp {
        "男职工" => PersonType::Male,
        "原法定退休年龄55周岁女职工" => PersonType::Female55,
        "原法定退休年龄50周岁女职工" => PersonType::Female50,
        _ => panic!("无效的人员类型"),
    };
    
    // 获取退休参数
    let (original_age, delay_start_year, delay_per_months, target_age) = match person_type {
        PersonType::Male => (60, 2025, 4, 63),
        PersonType::Female55 => (55, 2025, 4, 58),
        PersonType::Female50 => (50, 2025, 2, 55),
    };
    
    // 计算出生总月数
    let birth_total_months = birth_date.year * 12 + birth_date.month;
    // 计算原始退休总月数
    let original_retire_months = birth_total_months + original_age * 12;
    
    // 计算从2025年1月起的经过周期数和延迟月数
    let delay_start_months = delay_start_year * 12; // 2025年1月的总月数
    let mut delay_months = 0;
    if original_retire_months >= delay_start_months {
        let months_since_delay_start = original_retire_months - delay_start_months;
        // 计算延迟周期数（每delay_per_months个月为一个周期）
        delay_months = (months_since_delay_start + delay_per_months - 1) / delay_per_months;
        
        // 限制最大延迟月数
        let max_delay = (target_age - original_age) * 12;
        if delay_months > max_delay {
            delay_months = max_delay;
        }
    }
    
    // 计算最终退休总月数
    let final_retire_months = original_retire_months + delay_months;
    let final_year = final_retire_months / 12;
    let final_month = final_retire_months % 12;
    let (final_year, final_month) = if final_month == 0 {
        (final_year - 1, 12)
    } else {
        (final_year, final_month)
    };
    
    // 计算精确年龄
    let age_months = final_retire_months - birth_total_months;
    let age = age_months as f64 / 12.0;
    
    // 格式化年龄：如果是整数则不显示小数部分
    let age_str = if age.fract() == 0.0 {
        format!("{}", age as i32)
    } else {
        format!("{:.2}", age)
    };
    
    // 格式化输出
    format!(
        "{}-{:02},{},{}",
        final_year,
        final_month,
        age_str,
        delay_months
    )
}