mod sum_lib;

fn main() {
    println!("I wanna drive my bicycle");
}

#[test]
fn unsignedoverflow_modes() {
    assert_eq!(sum_lib::add_u8_checked(255, 1), None);
    assert_eq!(sum_lib::add_u8_wrapping(255, 1), 0);
    assert_eq!(sum_lib::add_u8_saturating(255, 1), 255);

    assert_eq!(sum_lib::add_u8_checked(10, 20), Some(30));
    assert_eq!(sum_lib::add_u8_wrapping(10, 20), 30);
    assert_eq!(sum_lib::add_u8_saturating(10, 20), 30);
}
