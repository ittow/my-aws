use std;

mod support;

fn _d008() {
    // Viết code chuyển hex 0x00000000 sang rgba(255, 255, 255, 255)
    let color_hex: String = "74ff4a90".to_lowercase();
    let mut color: [u8; 8] = [0; 8];
    for (i, c) in color_hex.chars().enumerate() {
        let num8: u8 = support::_hex8_to_num(c);
        color[i] = num8;
    }

    let mut rgba: [u8; 4] = [0; 4];

    for (index, _) in color.iter().enumerate() {
        // Chỉ tính số lẻ
        if index % 2 != 0 {continue;}

        let arr16: [u8; 2] = [color[index], color[index+1]];
        rgba[index / 2] = arr16[0] * 16 + arr16[1];
    }

    println!("Color: {:?}", rgba);
    println!("Red Green Blue Alpha: rgba({}, {}, {}, {})", rgba[0], rgba[1],  rgba[2],  rgba[3]);

    // Thử tính year từ ngày nhưng không dùng loop

    let days: i64 = 15000;
    let years1: [i64; 2] = [(days as f64 / 365.25) as i64 + 1, (days as f64 % 365.25) as i64];
    let years2: [i64; 2] = support::_year_and_day(days);

    println!("{:?}", years1);
    println!("{:?}", years2);
    // Sẽ cố độ lệch ở năm nhuận, điều này khiến số ngày không chính xác
}

fn _d009() {
    // Viết thuận toán giải hệ số x về hệ số 10
    // Công thức ví dụ S_k = a * k ** 2 + a * k ** 1 + a * k ** 0

    let mut str_sys_num: String = String::new();
    let read_sysnum: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut str_sys_num);
    if let Err(read_error) = read_sysnum {
        println!("Lỗi đọc dữ liệu: {}", read_error);
        return;
    }
    
    if !support::_isdigu(&str_sys_num) {
        println!("{} không phải số nguyên dương.", str_sys_num.trim());
        return;
    }
    
    let mut str_number: String = String::new();
    let read_number: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut str_number);
    if let Err(read_error) = read_number {
        println!("Lỗi đọc dữ liệu: {}", read_error);
        return;
    }
    
    let k_number: u128 = match str_sys_num.trim().parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Lỗi chuyển sang số nguyên: {}", error);
            return;
        }
    };
    
    if !(k_number <= 35) {
        println!("Hệ só {} không được hỗ trợ.", k_number);
        return;
    }
    
    let length: usize = str_number.trim().chars().count() - 1;
    let mut total: u128 = 0;
    for (index , char) in str_number.trim().chars().enumerate() {
        total += support::_hex8_to_num(char) as u128
        * k_number.pow((length - index) as u32);
    }
    
    println!("Tatal 10: {}", total);
    {
        let mut word: u8 = 74;  // 01001010 = 74
        
        let mut shift: u8 = 2;
        //  Kết quả mong muốn: 01001110 = 78
        support::_set(&mut word, shift);
        println!("Result: {0} / Bit: {0:0b}", word);

        shift = 1;
        //  Kết quả mong muốn: 01001100 = 76
        support::_clear(&mut word, shift);
        println!("Result: {0} / Bit: {0:0b}", word);

        shift = 0;
        //  Kết quả mong muốn: 01001101 = 77
        support::_toggle(&mut word, shift);
        println!("Result: {0} / Bit: {0:0b}", word);

        shift = 7;
        //  Kết quả mong muốn: 00000000 = 0
        support::_read(&mut word, shift);
        println!("Result: {0} / Bit: {0:0b}", word);
    }
}

fn _d010() {
    let string: String = String::from(" < \"What are\\\n you\\\" doing?\">");
    let substr: Option<&str> = support::_parse_string(&string);
    println!("Substring: {:?}", substr);

    let mut list_search: [&str; 50] = [
        "Google", "Genshin Impact", "Map", "Earth", "Music",
        "Minecraft Java", "Hello, world!", "Python", "Human", "Youtube",
        "Rust", "T-Red", "ASMR", "Google Earth", "Map Minecraft",
        "Homosapien", "Gmail", "Wikipedia", "Github", "Images",
        "Anime", "Video", "Assembly", "Honkai Impact 3rd", "Honkai Star Rail",
        "Minecraft Bedrock", "Wikimedia", "Star wars", "Animator", "Alan Walker",
        "Green Earth", "Zombie", "Fast", "Fact", "Alan Becker",
        "Google Map", "Microsoft", "Apple", "Bad Apple!!", "Moon",
        "Profile", "Project", "Object", "Drive", "Alone",
        "Hello you", "Java", "JavaScript", "Animation", "Google Search"
    ];

    let mut keyword: String = String::new();
    let read_keyword: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut keyword);
    if let Err(read_error) = read_keyword {
        println!("Lỗi đọc dữ liệu: {}", read_error);
        return;
    }

    let result: Vec<&str> = support::_search_from_list(&keyword.trim(), list_search.to_vec());
    println!("Result: {:?}", result);

    support::_sorted_str(&mut list_search);
    println!("Sorted str: {:?}", list_search);

    let mut array_i64: [i64; 10] = [9, 3, 6, 5, 4, 7, 2, 1, 8, 0];
    support::_sorted_i64(&mut array_i64);
    println!("Sorted i64: {:?}", array_i64);

    let mut array_u64: [u64; 10] = [8342, 392, 535, 72398, 202, 11, 3872, 5938, 482, 0];
    support::_sorted_u64(&mut array_u64);
    println!("Sorted u64: {:?}", array_u64);

    let mut array_f64: [f64; 10] = [0.2, 3.3, 2.1, 4.9, 9.2, 6.5, 2.3, 7.8, 5.6, 1.8];
    support::_sorted_f64(&mut array_f64);
    println!("Sorted f64: {:?}", array_f64);

    let fibonacci: u64 = support::_fibonacci(5);
    println!("Fibonacci: {}", fibonacci);
}

fn main() {
    let start: std::time::Instant = support::_hello();
    // Code here
    _d010();
    support::_the_end(start);
}