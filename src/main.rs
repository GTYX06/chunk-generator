fn main() {
    let mut x: i8 = 127;
    while x != -128 {
        println!("{}",x);
        x-=1;
    }
}
