pub use human::{Female, Man, EchoPerson};
pub use http_client::errors;

pub mod human;
pub mod http_client;

pub fn yolo(){
    let a = Man{name: "yolo"};
    let b = a.echo();

    println!("Welcome to Echo {b}")
}
