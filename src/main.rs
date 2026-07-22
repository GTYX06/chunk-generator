use std::io;

fn main() {
    let A = 7;
    let B = A * 3;
    println!("A times 3 = {}",B);
    let mut C: i16 = -256;
    while C != 255 {
        println!("{}",C);
        C+=1
    }
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input.");

    println!("You typed: {}",input)
}
