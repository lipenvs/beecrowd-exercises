use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please type a number!");
    for i in 0..input {
        if i == input - 1 {
            println!("Ho!");
        } else {
            print!("Ho ");
        }
    }
}
