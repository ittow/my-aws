use chrono::NaiveDate;
use chrono::NaiveDateTime;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Error;
use std::path::Path;

mod support;

fn _d005() {
    let result_ymd: Option<NaiveDate> = NaiveDate::from_ymd_opt(2025, 2, 1);

    let date: NaiveDate = match result_ymd {
        Some(dt) => dt,
        None => {
            println!("Error: d005 - result ymd: Thời gian không hợp lệ");
            return;
        }
    };

    let result_hms: Option<chrono::NaiveDateTime> = date.and_hms_opt(17, 24, 59);

    let datetime: NaiveDateTime = match result_hms {
        Some(dt) => dt,
        None => {
            println!("Error: d005 - result hms: Thời gian không hợp lệ");
            return;
        }
    };

    println!("{}", datetime);

    let path: &Path = Path::new("src/text.txt");

    if !path.exists() {
        println!("Path Error: {} Không tồn tại!", path.display());
        return;
    } else {
        println!("Path exists!")
    }

    if path.is_file() {
        println!("Path là file!");
    } else if path.is_dir() {
        println!("Path là dir!");
    }

    let file_name: Option<&std::ffi::OsStr> = path.file_name();
    let extension: Option<&std::ffi::OsStr> = path.extension();
    let dirparent: Option<&Path> = path.parent();

    println!("File name: {:?}", file_name);
    println!("Extension: {:?}", extension);
    println!("Parent: {:?}", dirparent);

    let result: Result<File, Error> = File::open(path);

    let mut file: File = match result {
        Ok(f) =>  f,
        Err(e) => {
            println!("File Error: {}", e);
            return;
        }
    };

    let mut butter: String = String::new();
    let _ = file.read_to_string(&mut butter);

    println!("{}", butter);

    // Một chương trình nhập số nguyên (giây) ra DD HH:MM:SS
    let mut string_seconds: String = String::new();
    io::stdin().read_line(&mut string_seconds)
        .expect("Không thể đọc dữ liệu");

    println!("stdin - string_seconds: {:?}", string_seconds);
    let isdigit: bool = support::_isdigi(&string_seconds.to_string());
    println!("isdigit: {}", isdigit);
    if !isdigit {
        println!("Đây không phải số nguyên!");
        return;
    }

    let total_seconds: u64 = string_seconds.trim().parse().expect("Có lỗi xảy ra");
    println!("Number: {}", total_seconds);

    let days: u64 = total_seconds / 86400;
    let hours: u64 = total_seconds % 86400 / 3600;
    let mins: u64 = total_seconds % 86400 % 3600 / 60;
    let secs: u64 = total_seconds % 86400 % 3600 % 60;

    println!("{:02} {:02}:{:02}:{:02}", days, hours, mins, secs);
}

fn main() {
    support::_hello();
    _d005();
}