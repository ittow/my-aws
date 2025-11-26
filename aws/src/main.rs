use std;

mod support;

fn _d007() {
    let mut str_seconds: String = String::new();
    let resopt: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut str_seconds);
    if let Err(error) = resopt {
        println!("Có lỗi đọc dữ liệu: {}", error);
        return;    
    }

    let isdigit: bool = support::_isdigu(&str_seconds);
    if !isdigit {
        println!("Đây không phải số nguyên dương -> {}", str_seconds);
        return;
    }

    let total_seconds: i64 = match str_seconds.trim().parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Lỗi xử lý chuyển đổi số nguyên: {}", error);
            return;
        }
    };

    println!("Total seconds: {}", total_seconds);
    support::_seconds_format(total_seconds);
    let s: &str = "<tag><chi></chi></tag>";
    let mut _tag_name: String = String::from("");

    let i: usize = 0;
    let a: char = match s.chars().nth(i) {
        Some(value) => value,
        None => return
    };

    if a == '<' {
        let j: usize= match s.find('>') {
            Some(value) => value,
            None => return
        };

        let _tn: &str = match s.get(i+1..j) {
            Some(value) => value,
            None => return
        };

        _tag_name.push_str(_tn);
    }

    println!("{}", _tag_name);
    // Việc phân tích cấu trúc như thẻ khá là mệt chủ yếu là lệnh khá dài dòng
}

fn main() {
    let start: std::time::Instant = support::_hello();
    // Code here
    _d007();
    support::_the_end(start);
}