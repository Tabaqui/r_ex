#[allow(dead_code)]
pub fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    let sum = a as u16 + b as u16;
    if sum > std::u8::MAX as u16 {
        return None 
    }
    return Some(sum as u8);
}

#[allow(dead_code)]
pub fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    let sum = a as u16 + b as u16;
    if sum > std::u8::MAX as u16 {
        return (sum % (std::u8::MAX as u16 + 1)) as u8
    } 
    return sum as u8;
}

#[allow(dead_code)]
pub fn add_u8_saturating(a: u8, b: u8) -> u8 {
    let sum = a as u16 + b as u16;
    if sum > std::u8::MAX as u16 {
        return std::u8::MAX
    } 
    return sum as u8;
}
