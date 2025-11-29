use chrono;
use std;

/// Kiểm tra một chuỗi là số nguyên dương
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

/// Kiểm tra năm nhuận
pub fn _is_leap_year(year: i64) -> bool {
    return year % 4 == 0
    && (year % 100 != 0
    || year % 400 == 0)
}

/// Chuyển số tháng thành tên tháng nhận từ 0-11
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

// Trả về hậu tố ngày st, nd, rd, th
pub fn _suffix_day(day: i64) -> &'static str {
    if 11 <= day && day <= 13 {
        return "th";
    }

    let sfx: &str = match day % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
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
    let suffix_day: &str = _suffix_day(days);
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
        '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5,
        '6' => 6, '7' => 7, '8' => 8, '9' => 9, 'a' => 10, 'b' => 11,
        'c' => 12, 'd' => 13, 'e' => 14, 'f' => 15, 'g' => 16, 'h' => 17,
        'i' => 18, 'j' => 19, 'k' => 20, 'l' => 21, 'm' => 22, 'n' => 23,
        'o' => 24, 'p' => 25, 'q' => 26, 'r' => 27, 's' => 28, 't' => 29,
        'u' => 30, 'v' => 31, 'w' => 32, 'x' => 33, 'y' => 34, 'z' => 35,
        _ => 0,
    };

    return n;
}

// Thực hành thao tác bit

pub fn _set(word: &mut u8, shift: u8) {
    // 00110110 bây giở cần đặt bit 1 ở vị trí 3
    // 00001000 cần shift 1 << 3
    // --------
    // 00111110
    // Vậy ta sẽ dùng OR
    *word |= 1 << shift;
}

pub fn _clear(word: &mut u8, shift: u8) {
    // 00110110 bây giờ cần xóa bit 1 ở vị trí 1
    // 00000010 cần shift << 1
    // -------- ?

    // 00110110
    // 11111101 vậy cần đảo bit
    // Các bit cùng đúng sẽ được giữ lại

    // Vậy ta sẽ dùng and + not
    *word &= !1 << shift;
}

pub fn _toggle(word: &mut u8, shift: u8) {
    // Về logic rất cơ bản
    // Chuyển 0 thành 1 và 1 thành 0 và ngược lại
    // Ta sẽ chỉ dùng xor
    *word ^= 1 << shift;
}

pub fn _read(word: &mut u8, shift: u8) {
    // Cái này cũng khá đơn giản
    // Ta shift bit cần đọc về 1
    // Sau đó dùng AND với 1 ta sẽ được giá trị 0 hoặc 1
    *word = 1 & (*word >> shift);
}

pub fn _parse_string(string: &str) -> Option<&str> {
    let length: usize = string.chars().count();

    let mut in_string: bool = false;
    let mut index: usize = 0;
    let mut jndex: usize = 0;
    let substr: Option<&str> = None;
    while index < length {
        let chars: char = match string.chars().nth(index) {
            Some(value) => value,
            None => {return substr;}
        };

        if chars == '\"'  {
            if index != 0 {
                let is_escape: char = match string.chars().nth(index-1) {
                    Some(value) => value,
                    None => {return substr;}
                };

                // Bỏ qua dấu " được escape
                if is_escape == '\\' {
                    index += 1;
                    continue;
                }
            }

            in_string = !in_string;
            if !in_string {
                let sub: Option<&str> = string.get(jndex+1..index);
                return sub;
            }
            jndex = index;
        }
        
        index += 1;
    }
    return substr;
}

pub fn _search_from_list(keyword: &str, list_search: Vec<&'static str>) -> Vec<&'static str> {
    let mut length: usize = 0;
    for search in list_search.iter() {
        if search.contains(keyword) {
            length += 1;
        }
    }

    let mut result: Vec<&str> = vec![""; length];
    let mut index: usize = 0;
    for search in list_search.iter() {
        if search.trim().contains(keyword) {
            result[index] = search;
            index += 1;
        }
    }

    return result;
}

pub fn _sorted_str(array_str: &mut [&str]) {
    let length: usize = array_str.len();
    for i in 0..length-1 {
        for j in 0..length - i - 1 {
            let temp: &str = array_str[j];
            if array_str[j] > array_str[j+1] {
                array_str[j] = array_str[j+1];
                array_str[j+1] = temp;
            }
        }
    }
}

pub fn _sorted_i64(array_i64: &mut [i64]) {
    let length: usize = array_i64.len();
    for i in 0..length-1 {
        for j in 0..length - i - 1 {
            let temp: i64 = array_i64[j];
            if array_i64[j] > array_i64[j+1] {
                array_i64[j] = array_i64[j+1];
                array_i64[j+1] = temp;
            }
        }
    }
}

pub fn _sorted_u64(array_u64: &mut [u64]) {
    let length: usize = array_u64.len();
    for i in 0..length-1 {
        for j in 0..length - i - 1 {
            let temp: u64 = array_u64[j];
            if array_u64[j] > array_u64[j+1] {
                array_u64[j] = array_u64[j+1];
                array_u64[j+1] = temp;
            }
        }
    }
}

pub fn _sorted_f64(array_f64: &mut [f64]) {
    let length: usize = array_f64.len();
    for i in 0..length-1 {
        for j in 0..length - i - 1 {
            let temp: f64 = array_f64[j];
            if array_f64[j] > array_f64[j+1] {
                array_f64[j] = array_f64[j+1];
                array_f64[j+1] = temp;
            }
        }
    }
}

pub fn _factorial(n: usize) -> u64 {
    let mut total: u64 = 1;
    for i in 1..n {
        total *= i as u64;
    }
    return total;
}

pub  fn _fibonacci(n: u64) -> u64 {
    if n <= 2 {
        return n;
    }

    return _fibonacci(n-1) + _fibonacci(n-2);
}