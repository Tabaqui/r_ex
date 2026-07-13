use vectorian::MyVec;

// use lib;
fn main() {
    println!("Hello, world!");
    let _v= make_v();
}

fn make_v() -> MyVec<u32> {
    MyVec::new()
}