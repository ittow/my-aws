use std::fs;

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
    for v in s.chars() {
        if !('0' <= v && v <= '9') {return false;}
    }
    return true;
}

pub fn _isdigf(s: &str) -> bool {
    for (i, v) in s.chars().enumerate() {
        if i == 0 && v == '.' || i == s.chars().count() - 1 && v == '.' {continue}
        if !('0' <= v && v <= '9') {return false;}
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

pub fn _filesystem() -> String {
    const PATH: &str = "src/text.txt";
    let content: String = fs::read_to_string(PATH)
        .expect("Có lỗi xảy ra: có thể file không tồn tại, hoặc đường dẫn không đúng.");

    return content;
}

pub fn _count(string: &str, sep: &str) -> usize {
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = string[start..].find(sep) {
        count += 1;
        start += pos + sep.len();
    }
    return count;
}

pub fn _split<'a>(array: &mut [&'a str], string: &str, sep: &str) {
    array[0] = "H";
    println!("count >> {} | {} | {}", array.len(), string, sep);
}