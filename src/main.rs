use std::io;
use nim::keys::{generate_keys, to_hex, from_hex, read_32, PublicKey, SecretKey};
use nim::crypto::{encrypt, decrypt};

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
            1 => {
                let (sk, pk) = generate_keys();
                println!("Secret Key: {}", to_hex(&sk.0));
                println!("Public Key: {}", to_hex(&pk.0));
            }
            2 => {
                let sk = match read_32(&ask("A tua chave privada (hex):")) {
                    Ok(b) => SecretKey::new(b),
                    Err(e) => { println!("{}", e); continue; }
                };
                let pk = match read_32(&ask("Chave pública do teu amigo (hex):")) {
                    Ok(b) => PublicKey::new(b),
                    Err(e) => { println!("{}", e); continue; }
                };
                let msg = ask("Mensagem:");

                match encrypt(&sk, &pk, &msg) {
                    Ok(bytes) => println!("Mensagem encriptada: {}", to_hex(&bytes)),
                    Err(e) => println!("Erro: {}", e),
                }
            }
            3 => {
                let sk = match read_32(&ask("A tua chave privada (hex):")) {
                    Ok(b) => SecretKey::new(b),
                    Err(e) => { println!("{}", e); continue; }
                };
                let pk = match read_32(&ask("Chave pública de quem enviou (hex):")) {
                    Ok(b) => PublicKey::new(b),
                    Err(e) => { println!("{}", e); continue; }
                };
                let data = match from_hex(&ask("Mensagem encriptada (hex):")) {
                    Ok(d) => d,
                    Err(e) => { println!("{}", e); continue; }
                };

                match decrypt(&sk, &pk, &data) {
                    Ok(text) => println!("Mensagem: {}", text),
                    Err(e) => println!("Erro: {}", e),
                }
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Opcao invalida"),
        }

    }

        
}


fn ask(prompt: &str) -> String {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Erro a ler");
    s.trim().to_string()
}