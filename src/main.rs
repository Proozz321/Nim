use std::io;

fn main() {
    println!("\n\n");

    loop {
        println!("Choose an option:");
        println!("[1] - Generate Private and Public keys");
        println!("[2] - Encrypt your own message");
        println!("[3] - Decrypt your friend's message");
        println!("[4] - Exit");

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
            1 => println!("funcao gerar chaves"),
            2 => println!("funcao encriptar mensagem"),
            3 => println!("funcao decriptar mensagem"),
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Opcao invalida"),
        }

    }

        
}
