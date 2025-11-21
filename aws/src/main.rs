// #![no_std]

use std::io;
mod support;

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
    let count: usize = support::_count(string, SEP);
    println!("{count}");

    let mut array: [&str; 5] = [""; 5];
    support::_split(&mut array, string, SEP);

    for str in array {
        println!("{str}");
    }
}

fn main() {
    _d003();
}