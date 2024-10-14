use std::io;

fn invert_the_case(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    println!("Введіть слово:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");
    let trimmed_input = input.trim();

    let inverted = invert_the_case(trimmed_input);
    println!("Inverted case of '{}': {}", trimmed_input, inverted);
}
