
use std::io;

fn main() {

    println!("Digite um texto: ");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    println!("Você digitou  o texto {s}");
    println!("Você digitou {} carcteres", s.trim().chars().count());

}
