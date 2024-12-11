use learning_rust::{Man, EchoPerson};

fn yolo() -> (u8, u8){
    return (5, 5)
}

fn main() {
    let efdal = Man{name: "Efdal Sancak"};
    efdal.echo();

    let (num1, num2) = yolo();
    println!("{num1}/{num2}=0")
}