use std::io;

fn main() {
    let prices = vec![
        (1, 4.00), // Item 1 custa R$ 4.00
// Item 1 custa R$ 4.00
        (2, 4.50), // Item 2 custa R$ 4.50
// Item 2 custa R$ 4.50
        (3, 5.00), // Item 3 custa R$ 5.00
// Item 3 custa R$ 5.00
        (4, 2.00), // Item 4 custa R$ 2.00
// Item 4 custa R$ 2.00
        (5, 1.50), // Item 5 custa R$ 1.50
// Item 5 custa R$ 1.50
    ];

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let code: i32 = values[0].parse().expect("Failed to parse code");
    let quantity: f64 = values[1].parse().expect("Failed to parse quantity");
    let mut unit_value = 0.0;

    for item in prices.iter() {
        if item.0 == code {
            unit_value = item.1;
            break;
        }
    }

    let total = unit_value * quantity;

    println!("Total: R$ {:.2}", total);
}