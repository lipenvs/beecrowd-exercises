use std::io;

fn main() {
    let mut entradas = String::new();

    io::stdin()
        .read_line(&mut entradas)
        .expect("Failed to read line");

    let entradas = entradas
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let maior = entradas.iter().max().expect("Failed to find max value");

    println!("{} eh o maior", maior);
}
