use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Utc;
use std::fs;
use std::time;

pub fn _add(a: i64, b: i64) -> i64 {
    return a + b;
}

pub fn _sqal(n: f64, b: i8) -> f64 {
    let mut t = n;
    for _ in 1..b {
        t *= n;
    }
    return t;
}

pub fn _isdigi(s: &str) -> bool {
    for chr in s.chars() {
        if !('0' <= chr && chr <= '9')
        && !(chr == '\r' || chr == '\n') {
            return false;
        }
    }
    return true;
}

pub fn _isdigf(s: &str) -> bool {
    for (i, chr) in s.chars().enumerate() {
        if i == 0 && chr == '.' || i == s.chars().count() - 1 && chr == '.' {continue}
        if !('0' <= chr && chr <= '9')
        && !(chr == '\r' || chr == '\n') {
            return false;
        }
    }
    return true;
}

pub fn _is_leap_year(year: i64) -> bool {
    return year % 4 == 0
    && (year % 100 != 0
    || year % 400 == 0)
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

pub fn _filesystem() -> String {
    const PATH: &str = "src/text.txt";
    let content: String = fs::read_to_string(PATH)
        .expect("Có lỗi xảy ra: có thể file không tồn tại, hoặc đường dẫn không đúng.");

    return content;
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

fn _process(begin: NaiveDateTime) -> String {
    let start: i64 = begin.and_utc().timestamp();
    let now: i64 = Utc::now().timestamp();

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

pub fn _hello() -> time::Instant {
    let start: time::Instant = time::Instant::now();

    let begin_ymd: Option<NaiveDate> = NaiveDate::from_ymd_opt(2025, 2, 1);
    let b_ymd_hms: Option<NaiveDateTime> = match begin_ymd {
        Some(k) => k.and_hms_opt(17, 24, 59),
        None => {return start;}
    };

    let begin: NaiveDateTime = match b_ymd_hms {
        Some(k) => k,
        None => {return start;}
    };

    let start_programming: String = _process(begin);
    println!("Start programming: {}", start_programming);

    let brust_ymd: Option<NaiveDate> = NaiveDate::from_ymd_opt(2025, 11, 1);
    let r_ymd_hms: Option<NaiveDateTime> = match brust_ymd {
        Some(k) => k.and_hms_opt(14, 36, 9),
        None => {return start;}
    };

    let brust: NaiveDateTime = match r_ymd_hms {
        Some(k) => k,
        None => {return start;}
    };

    let start_with_rust: String = _process(brust);
    println!("Start with Rust: {}", start_with_rust);
    println!("Hello World");
    
    return start;
}

pub fn _the_end(start: time::Instant) {
    let elapsed: u128 = start.elapsed().as_micros();
    println!("Running in {} µs", elapsed);
}

pub fn _title_month(nummon: i64) -> String {
    let res: String = match nummon {
        1 => String::from("Jan"),
        2 => String::from("Feb"),
        3 => String::from("Mar"),
        4 => String::from("Apr"),
        5 => String::from("May"),
        6 => String::from("Jun"),
        7 => String::from("Jul"),
        8 => String::from("Aug"),
        9 => String::from("Sep"),
        10 => String::from("Oct"),
        11 => String::from("Nov"),
        12 => String::from("Dec"),
        _ => String::from("Jan")
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

pub fn _total_leap_year(year: i64) -> i64 {
    let y4: i64 = year / 4;
    let y100: i64 = year / 100;
    let y400: i64 = year / 400;

    return y4 + y400 - y100;
}

pub fn _num_month(day: i64, year: i64) -> [i64; 2] {
    let mut _months: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if _is_leap_year(year) {
        _months[1] = 29;
    }

    let mut t: [i64; 2] = [0; 2];
    let mut d: i64 = 0;
    for (i, v) in _months.iter().enumerate() {
        d += v;
        if day < d {
            t = [i as i64, d - v];
            return t;
        }
    }

    return t;
}

pub fn _year_and_day(day: i64) -> [i64; 2] {
    let mut d: i64 = day;
    let mut year: i64 = 0;
    loop {
        let is_leap_year: bool = _is_leap_year(year);
        if is_leap_year && d > 366 {
            d -= 366;
            year += 1;
        } else
        if !is_leap_year && d > 365 {
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
            t = [i as i64 + 1, d + v];
            return t;
        }
    }

    return t;
}