use std::io;

fn main() {
    println!("お名前を入力");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("こんにちは! {name}");

    let (x,y) = (100, 200);
    const CONSTANT :i32= 50;
    println!("x={x},y={y},CONSTANT={CONSTANT}");
}
