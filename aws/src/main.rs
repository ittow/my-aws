use std::io;
mod support;

fn d001() {
    let mut s: String = String::from("Hello, world");
    println!("{}", s);

    s.push_str("!");    
    println!("String {}", s);
    println!("Size: {}", s.len());
    println!("Length: {}", s.chars().count());

    let a: i64 = 10;
    let b: i64 = 20;

    let c: i64 = support::add(a, b);
    println!("Total: {}", c);

    let mut i: String = String::new();
    io::stdin().read_line(&mut i).expect("Hãy nhập gì đó");
    if i == "" { print!("i = {i}"); }

    let t: &str = "100.";
    let b1: bool = support::isdigi(t);
    let b2: bool = support::isdigf(t);
    if b1 == true { println!("t là số nguyên"); }
    if b2 == true { println!("t là số thực"); }

    let x: f64 = support::sqal(2 as f64, 5);
    println!("x = {x}");
}

fn main() {
    d001();
}
