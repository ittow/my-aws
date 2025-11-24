use std::io;
use std::time;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Utc;

fn _d001() {
    let mut s: String = String::from("Hello, world");
    println!("{}", s);

    s.push_str("!");
    println!("String {}", s);
    println!("Size: {}", s.len());
    println!("Length: {}", s.chars().count());

    let a: i64 = 10;
    let b: i64 = 20;

    let c: i64 = support::_add(a, b);
    println!("Total: {}", c);

    let mut i: String = String::new();
    io::stdin().read_line(&mut i).expect("Hãy nhập gì đó");
    if i == "" { print!("i = {i}"); }

    let t: &str = "100.";
    let b1: bool = support::_isdigi(t);
    let b2: bool = support::_isdigf(t);
    if b1 == true { println!("t là số nguyên"); }
    if b2 == true { println!("t là số thực"); }

    let x: f64 = support::_sqal(2 as f64, 5);
    println!("x = {x}");
}

fn _d002() {
    let mut array: [i64; 10] = [3, 7, 4, 9, 2, 8, 5, 6, 0, 1];
    support::_lrsorted(&mut array);
    for a in array { print!("{a} ") }
    println!();
    
    support::_rlsorted(&mut array);
    for a in array { print!("{a} ") }
    println!();

    let d: String =  support::_filesystem();
    println!("{d}");
}

fn _d003() {
    const SEP: &str = " ";
    let string: &str = "Xin chào thế giới";
    support::_count(string, SEP);
}

fn _d004() {
    // support::_hello();

    // let start: time::Instant = time::Instant::now();
    // println!("{:?}", start.elapsed());

    // let duration: time::Duration = time::Duration::from_secs(100);
    // println!("{:?}", duration);

    let begin: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 11, 24).unwrap().and_hms_opt(13, 9, 0).unwrap();
    // let begin: NaiveDateTime = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap().and_hms_opt(17, 24, 59).unwrap();
    // let brust: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 11, 1).unwrap().and_hms_opt(14, 36, 9).unwrap();

    let now: DateTime<Utc> = Utc::now();

    // println!("{}", ndt);
    // println!("{}", ndt.time());
    // println!("{}", ndt.date());
    // println!("{}", ndt.second());
    // println!("{}", ndt.and_utc().timestamp());

    // println!("{}", now.timestamp());

    let start: i64 = begin.and_utc().timestamp();
    // let rstart: i64 = brust.and_utc().timestamp();

    let end: i64 = now.timestamp();
    // println!("{}", end - start);
    let delta: i64 = end - start;
    // let rdelta: i64 = end - rstart;

    let year: i64 = delta / 31557600;
    let day: i64 = delta % 31557600 / 86400;
    let hour: i64 = delta % 31557600 % 86400 / 3600;
    let min: i64 = delta % 31557600 % 86400 % 3600 / 60;
    let sec: i64 = delta % 31557600 % 86400 % 3600 % 60;

    // let ryear: i64 = rdelta / 31557600;
    // let rday: i64 = rdelta % 31557600 / 86400;
    // let rhour: i64 = rdelta % 31557600 % 86400 / 3600;
    // let rmin: i64 = rdelta % 31557600 % 86400 % 3600 / 60;
    // let rsec: i64 = rdelta % 31557600 % 86400 % 3600 % 60;

    println!("Start programming: {year:02}:{day:02}:{hour:02}:{min:02}:{sec:02}");
    // println!("Start with Rust: {ryear:02}:{rday:02}:{rhour:02}:{rmin:02}:{rsec:02}:");
}