fn main() {
    let image = [
        "..####..",
        ".#....#.",
        "#.#..#.#",
        "#..##..#",
        "#......#",
        "#.#..#.#",
        ".#....#.",
        "..####..",
    ];

    let bytes = parse_bitmap_8x8(image);

    println!("Bytes:");
    for byte in bytes {
        println!("{byte:08b} 0x{byte:02X}");
    }

    println!();

    println!("Rendered:");
    for line in render_bitmap_8x8(bytes) {
        println!("{line}");
    }

    println!();
    
    println!("Inverted:");
    for line in render_bitmap_8x8(invert_bitmap_8x8(bytes)) {
        println!("{line}");
    }
}

fn parse_bitmap_8x8(image: [&str; 8]) -> [u8; 8] {
    let mapped: [u8; 8] = image.iter().map(|line| {
        let line_chars = line.chars();
        let mut mapped_byte: u8 = 0b00000000;
        line_chars.enumerate().for_each(|(i, i_char)|  {
        
            match i_char {
                '#' => { 
                    mapped_byte |= 1 << 7 - i
                }
                '.' => {
                    // be cool
                }
                _ => panic!("The input's not cool.")
            };

        });

        mapped_byte
    }).collect::<Vec<u8>>().try_into().unwrap();

    mapped
}

pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] { 
    let mapped = bytes.iter().map(|line| {
        let mut mapped_string = String::new();
        for bit_index in (0..=7).rev() {
            let is_set = line & (1 << bit_index) != 0;
            if is_set {
                mapped_string.push('#');
            } else {
                mapped_string.push('.');
            }
        }
        mapped_string
    }).collect::<Vec<String>>().try_into().unwrap();

    mapped
}

fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
     let mapped: [u8; 8] = bytes.iter().map(|line| {
        let mapped_byte: u8 = !line;
        mapped_byte
    }).collect::<Vec<u8>>().try_into().unwrap();
    mapped
}


#[test]
fn parse_zeroes() {
    let image = [
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
    ];

    let bytes = parse_bitmap_8x8(image);
    let sum: u8 = bytes.iter().sum();

    assert_eq!(sum, 0);
}

#[test]
fn parse_ones() {
    let image = [
        "########",
        "########",
        "########",
        "########",
        "########",
        "########",
        "########",
        "########",
    ];

    let bytes: [u8; 8] = parse_bitmap_8x8(image);
    let inv_sum: u8 = bytes.iter().map(|b| !b).sum();

    assert_eq!(inv_sum, 0);
}

#[test]
fn parse_asymmetric() {
    let image = [
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
    ];

    let bytes = parse_bitmap_8x8(image);
    let sum: u8 = bytes.iter().sum();

    assert_eq!(sum, 8);
}

#[test]
fn test_render_asymmetric() {
    let image = [
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
    ];

    let bytes = parse_bitmap_8x8(image);
    let render = render_bitmap_8x8(bytes);
    
    for i in 0..=7 {
        assert_eq!(".......#", render[i]);
    }
}

#[test]
fn test_invert_asymmetric() {
    let image = [
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
        ".......#",
    ];

    let bytes = parse_bitmap_8x8(image);
    let invert = invert_bitmap_8x8(bytes);
    
    for i in 0..=7 {
        assert_eq!(0b11111110, invert[i]);
    }
}

