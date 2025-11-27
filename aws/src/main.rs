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

fn main() {
    let start: std::time::Instant = support::_hello();
    // Code here
    _d008();
    support::_the_end(start);
}