fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<String>>();
    let f = args.len() > 0;
    args.sort();
    args.iter().for_each(|s_v| println!("{}", s_v));
    if f {
        println!()
    }
    
}
