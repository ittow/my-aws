pub fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

pub fn sqal(n: f64, b: i8) -> f64 {
    let mut t = n;
    for _ in 1..b {
        t *= n;
    }
    return t;
}

pub fn isdigi(s: &str) -> bool {
    for v in s.chars() {
        if !('0' <= v && v <= '9') {return false;}
    }
    return true;
}

pub fn isdigf(s: &str) -> bool {
    for (i, v) in s.chars().enumerate() {
        if i == 0 && v == '.' || i == s.chars().count() - 1 && v == '.' {continue}
        if !('0' <= v && v <= '9') {return false;}
    }
    return true;
}