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
        let word: u8 = 74;  // 01001010 = 74
        let shift: u8 = 2;  // 00000100 = shift 2
        
        //  Kết quả mong muốn: 01001110 = 78
        let result: u8 = support::_set(word, shift);
        println!("Result: {0} / Bit: {0:0b}", result);
    }
    {
        let word: u8 = 74;  // 01001010 = 74
        let shift: u8 = 1;  // 00000010 = shift 1
        
        //  Kết quả mong muốn: 01001000 = 72
        let result: u8 = support::_clear(word, shift);
        println!("Result: {0} / Bit: {0:0b}", result);
    }
    {
        let word: u8 = 74;  // 01001010 = 74
        let shift: u8 = 3;  // 00001000 = shift 3
        
        //  Kết quả mong muốn: 01000010 = 66
        let result: u8 = support::_toggle(word, shift);
        println!("Result: {0} / Bit: {0:0b}", result);
    }
    {
        let word: u8 = 74;  // 01001010 = 74
        let shift: u8 = 6;  // 01000000 = shift 6
        
        //  Kết quả mong muốn: 00000001 = 1
        let result: u8 = support::_read(word, shift);
        println!("Result: {0} / Bit: {0:0b}", result);
    }
}

fn main() {
    let start: std::time::Instant = support::_hello();
    // Code here
    _d009();
    support::_the_end(start);
}