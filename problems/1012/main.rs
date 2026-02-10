use std::io;

fn main() {
    let mut entradas = String::new();

    io::stdin()
        .read_line(&mut entradas)
        .expect("Failed to read line");

    let entradas = entradas
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let area_triangulo = (entradas[0] * entradas[2]) / 2.0;
    let area_circulo = 3.14159 * entradas[2].powi(2);
    let area_trapezio = ((entradas[0] + entradas[1]) * entradas[2]) / 2.0;
    let area_quadrado = entradas[1].powi(2);
    let area_retangulo = entradas[0] * entradas[1];

    println!("TRIANGULO: {:.3}", area_triangulo);
    println!("CIRCULO: {:.3}", area_circulo);
    println!("TRAPEZIO: {:.3}", area_trapezio);
    println!("QUADRADO: {:.3}", area_quadrado);
    println!("RETANGULO: {:.3}", area_retangulo);
}