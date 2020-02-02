extern "C" {
    fn testing(x: u8) -> u8;
}

fn main() {
    let a = unsafe { testing(21) };
    println!("a = {}", a);
}