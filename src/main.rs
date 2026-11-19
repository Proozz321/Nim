use std::io;

fn main() {
    println!("\n\n");
    println!("Choose an option:");
    println!("[1] - Generate Private and Public keys");
    println!("[2] - Encrypt your own message");
    println!("[3] - Decrypt your friend's message");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error trying to read input");
    
    
    let num: i32 = match input.trim().parse() {
        Ok(a) => a,
        Err(_) => {
            println!("Not a valid number");
            return;
        }
    };

    match num {
        1 => println!("Choice 1"),
        2 => println!("Choice 2"),
        3 => println!("Choice 3"),
        _ => println!("Opcao invalida"),
    }
    
}
