use std::io;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");

    let input_qtd: u8 = input.trim().parse().expect("Invalid input");

    if input_qtd < 1 || input_qtd > 100 {
        return;
    }

    for _ in 0..input_qtd {
        input.clear();

        io::stdin().read_line(&mut input).expect("Error reading");
        let input_number: Result<i32, _> = input.trim().parse();
        match input_number {
            Ok(number) => {
                if is_prime(number) {
                    println!("{} eh primo", number);
                } else {
                    println!("{} nao eh primo", number);
                }
            }
            Err(_) => {
                println!("Entrada inválida! Por favor, insira um número inteiro.");
            }
        }
    }
}
