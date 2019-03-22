fn main() {
    println!("Hello, area {}!", multiply(17,3));
}

#[no_mangle]
pub extern "C" fn multiply(a: u32, b: u32) -> u32 {
    a * b
}
