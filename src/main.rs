fn main() {
    let A = 7;
    let B = A * 3;
    println!("A times 3 = {}",B);
    let mut C: i16 = -32768;
    while C != 32767 {
        println!("{}",C);
        C+=1
    }
}
