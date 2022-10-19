// Rust command line client for payment service
use figlet_rs::FIGfont;
use std::io;

mod client_api;
use client_api::{all_users, sign_in, close_con, get_balance, send_payment};

const EXIT: u32 = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Toy Wallet");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());

    // get client name
    let mut client_name_untrim = String::new();
    let mut client_name = String::new();

    loop {
        println!("Username:");
        // sanitize input

        io::stdin()
            .read_line(&mut client_name_untrim)
            .expect("Failed to read line");

        client_name = client_name_untrim.trim().into();
        if client_name.len() < 4 {
            client_name_untrim.clear();
            client_name.clear();
            println!("Please enter username with more than 3 characters");
            continue;
        }
        break;
    }

    sign_in(&mut client_name).await?;

    println!("This is your digital wallet");
    loop {
        println!("(1) check balance");
        println!("(2) transfer balance");
        println!("(3) directory");
        println!("({}) exit", EXIT);

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            EXIT => {
                close_con(&client_name).await?;
                break;
            }
            1 => get_balance(&client_name).await?,
            2 => send_payment(&client_name).await?,
            3 => all_users(&client_name).await?,
            _ => {
                println!("try again");
                String::from("try again")
            }
        };
        println!("\n");
    }
    Ok(())
}
