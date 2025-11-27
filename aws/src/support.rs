use chrono;
use std;

pub fn _isdigu(s: &str) -> bool {
    for chr in s.chars() {
        if !('0' <= chr && chr <= '9')
        && !(chr == '\r' || chr == '\n') {
            return false;
        }
    }
    return true;
}

pub fn _isdigf(s: &str) -> bool {
    let mut hasc: bool = false;
    for (idx, chr) in s.chars().enumerate() {

        // Nếu là dấu chấm
        if chr == '.' {
            // Nếu chưa từng có dấu chấm
            // Và vị trí dấu chấm không phải 0
            if !hasc && idx != 0 {
                hasc = true;
                continue;
            // Nếu đã có dấu chấm
            // Hoặc vị trí ở 0
            } else {
                return false;
            }
        }

        // Nếu không nằm từ 0 đến 9
        // Và Không phải ký tự điều khiển
        if !('0' <= chr && chr <= '9')
        && !(chr == '\r' || chr == '\n') {
            return false;
        }
    }
    return true;
}

pub fn _lrsorted(array: &mut [i64]) {
    let n: usize = array.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            let temp: i64 = array[j];
                if array[j] > array[j+1] {
                    array[j] = array[j+1];
                    array[j+1] = temp;
            }
        }
    }
}

pub fn _rlsorted(array: &mut [i64]) {
    let n: usize = array.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            let temp: i64 = array[j];
                if array[j] < array[j+1] {
                    array[j] = array[j+1];
                    array[j+1] = temp;
            }
        }
    }
}

pub fn _count(string: &str, sep: &str) -> usize {
    let mut count: usize = 0;
    let mut start: usize = 0;

    while let Some(pos) = string[start..].find(sep) {
        count += 1;
        start += pos + sep.len();
    }
    return count;
}

fn _process(begin: chrono::NaiveDateTime) -> String {
    let start: i64 = begin.and_utc().timestamp();
    let now: i64 = chrono::Utc::now().timestamp();

    let delta: i64 = now - start + 25200; // UTC + 7

    // Giây thứ 31536000 = 00:365:00:00:00
    // Giây thứ 31557600 = 01: 00:00:00:00
    let year: i64 = delta / 31557600;
    let day: i64 = delta % 31557600 / 86400;
    let hour: i64 = delta % 31557600 % 86400 / 3600;
    let min: i64 = delta % 3600 / 60;
    let sec: i64 = delta % 60;

    let fmt: String = format!("{year:02}:{day:02}:{hour:02}:{min:02}:{sec:02}");
    return fmt;
}

pub fn _hello() -> std::time::Instant {
    let start: std::time::Instant = std::time::Instant::now();

    let begin_ymd: Option<chrono::NaiveDate> = chrono::NaiveDate::from_ymd_opt(2025, 2, 1);
    let b_ymd_hms: Option<chrono::NaiveDateTime> = match begin_ymd {
        Some(value) => value.and_hms_opt(17, 24, 59),
        None => {return start;}
    };

    let begin: chrono::NaiveDateTime = match b_ymd_hms {
        Some(value) => value,
        None => {return start;}
    };

    let start_programming: String = _process(begin);
    println!("Start programming: {}", start_programming);

    let brust_ymd: Option<chrono::NaiveDate> = chrono::NaiveDate::from_ymd_opt(2025, 11, 1);
    let r_ymd_hms: Option<chrono::NaiveDateTime> = match brust_ymd {
        Some(value) => value.and_hms_opt(14, 36, 9),
        None => {return start;}
    };

    let brust: chrono::NaiveDateTime = match r_ymd_hms {
        Some(value) => value,
        None => {return start;}
    };

    let start_with_rust: String = _process(brust);
    println!("Start with Rust: {}", start_with_rust);
    println!("Hello, world!");
    
    return start;
}

pub fn _the_end(start: std::time::Instant) {
    let elapsed: u128 = start.elapsed().as_micros();
    println!("Running in {} µs", elapsed);
}

pub fn _is_leap_year(year: i64) -> bool {
    return year % 4 == 0
    && (year % 100 != 0
    || year % 400 == 0)
}

pub fn _title_month(month: i64) -> String {
    let res: String = match month {
        0 => String::from("January"),
        1 => String::from("February"),
        2 => String::from("March"),
        3 => String::from("April"),
        4 => String::from("May"),
        5 => String::from("June"),
        6 => String::from("July"),
        7 => String::from("August"),
        8 => String::from("September"),
        9 => String::from("October"),
        10 => String::from("November"),
        11 => String::from("December"),
        _ => String::from("January")
    };

    return res;
}

pub fn _suffix_day(day: i64) -> String {
    if 11 <= day && day <= 13 {
        return String::from("th");
    }

    let sfx: String = match day % 10 {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th")
    };

    return sfx;
}

pub fn _year_and_day(day: i64) -> [i64; 2] {
    let mut d: i64 = day;
    let mut year: i64 = 1;
    loop {
        let is_leap_year: bool = _is_leap_year(year);
        if is_leap_year && d >= 366 {
            d -= 366;
            year += 1;
        } else
        if !is_leap_year && d >= 365 {
            d -= 365;
            year += 1;
        } else {
            return [year, d];
        }
    }
}

pub fn _month_and_day(day: i64, year: i64) -> [i64; 2] {
    let mut _months: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if _is_leap_year(year) {
        _months[1] = 29;
    }

    let mut d: i64 = day;
    let mut t: [i64; 2] = [0; 2];

    for (i, v) in _months.iter().enumerate() {
        d -= v;
        if d < 0 {
            t = [i as i64, d + v];
            return t;
        }
    }

    return t;
}

pub fn _seconds_to_hms(seconds: i64) -> [i64; 3] {
    let hours: i64 = seconds / 3600;
    let mints: i64 = seconds % 3600 / 60;
    let secos: i64 = seconds % 60;

    return [hours, mints, secos];
}

pub fn _day_in_week(days: i64) -> String {
    let e: i64 = days % 7;
    let t: String = match e {
        0 => String::from("Monday"),
        1 => String::from("Tuesday"),
        2 => String::from("Wednesday"),
        3 => String::from("Thursday"),
        4 => String::from("Friday"),
        5 => String::from("Saturday"),
        6 => String::from("Sunday"),
        _ => String::from("Monday")
    };

    return t;
}

pub fn _seconds_format(seconds: i64) -> String {
    // Viết thuật toán nhập số ngày ra Month Day + suffix, Year
    let total_day: i64 = seconds / 86400;
    let t_seconds: i64 = seconds % 86400;
    let hms: [i64; 3] = _seconds_to_hms(t_seconds);
    let diw: String = _day_in_week(total_day);

    let year_and_day: [i64; 2] = _year_and_day(total_day);
    let month_and_day: [i64; 2] = _month_and_day(year_and_day[1], year_and_day[0]);

    let years: i64 = year_and_day[0];
    let days: i64 = month_and_day[1] + 1;
    let suffix_day: String = _suffix_day(days);
    let title_month: String = _title_month(month_and_day[0]);

    let hours: i64 = hms[0];
    let apm: &str = if hours < 12 {"AM"} else {"PM"};
    let mut hours_apm: i64 = hours % 12;

    if hours_apm == 0 {
        hours_apm = 12;
    }

    let fmt: String = format!("{:02}:{:02}:{:02} {} - {}, {} {}{}, {:04}", hours_apm, hms[1], hms[2], apm, diw, title_month, days, suffix_day, years);
    return fmt;
}


pub fn _hex8_to_num(c: char) -> u8 {
    let n: u8 = match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => 0,
    };

    return n;
}

pub fn _hexa2_to_num(a: [u8; 2]) -> u8 {
    return a[0] * 16 + a[1];
}
