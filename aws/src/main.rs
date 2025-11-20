use std::io;

use crate::support::_filesystem;
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

    _filesystem();
}

fn main() {
    _d002();
}