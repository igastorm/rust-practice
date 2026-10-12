use std::io;

fn main() {
    println!("お名前を入力");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("こんにちは! {name}");
}
