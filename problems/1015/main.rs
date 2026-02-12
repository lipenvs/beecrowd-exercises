use std::io;

fn main() {
    let mut line_one = String::new();

    io::stdin().read_line(&mut line_one).expect("Failed to read line");

    let v: Vec<&str> = line_one.trim().split(' ').collect();

    let x1:f64 = v[0].parse().expect("Invalid input");
    let y1:f64 = v[1].parse().expect("Invalid input");

    let mut line_two = String::new();

    io::stdin().read_line(&mut line_two).expect("Failed to read line");

    let v: Vec<&str> = line_two.trim().split(' ').collect();

    let x2:f64 = v[0].parse().expect("Invalid input");
    let y2:f64 = v[1].parse().expect("Invalid input");

    let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    println!("{:.4}", distance);


}
