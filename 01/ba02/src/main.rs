#[doc(inline)]
pub use std;

use std::io::Read;

fn main() {
    let bytes_std_in = std::io::stdin();

    let mut line_count = 0;

    let mut word_count = 0;
    let mut in_word = false;

    let mut byte_count = 0;
    bytes_std_in.bytes().for_each(|r| {
        let bt = r.expect("Wrong stream! Get yorself together");

        if bt == b'\n' {
            line_count += 1;
        }

        if bt != b'\n' && bt != b'\t' && !bt.is_ascii_whitespace() {
            if in_word != true {
                word_count += 1;
            }
            in_word = true;
        } else {
            if in_word == true {
                
                            // println!("2t");

            }
            in_word = false;
                        // println!("1f");

        }

        byte_count += 1;
    });

    println!("{} {} {}\n", line_count, word_count, byte_count);
    
}
