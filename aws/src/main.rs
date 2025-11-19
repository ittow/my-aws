mod support;

fn main() {
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
}
