use payments::bitcoin_client::BitcoinClient;
use payments::{BtcBalanceRequest, BtcExitInit, BtcSignIn, BtcPaymentRequest};
use std::io;

const EXIT: u32 = 3;
const SERVER: &str = "http://172.20.10.8:50052";

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Crypto Wallet V1.0");

    // get client name
    let mut client_name_untrim = String::new();
    let mut client_name = String::new();
    println!("Username:");

    io::stdin()
        .read_line(&mut client_name_untrim)
        .expect("Failed to read line");

    client_name = client_name_untrim.trim().into();
    sign_in(&mut client_name).await?;



    loop {
        println!("Welcome to your digital wallet");
        println!("(1) check your balance");
        println!("(2) send A-coin");
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
            _ => println!("try again"),
        }
        println!("\n");
    }
    Ok(())
}

async fn send_payment(client_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client_to_send_untrim = String::new();
    let mut client_to_send = String::new();
    println!("who would you like to send A-coin to?");

    io::stdin()
        .read_line(&mut client_to_send_untrim)
        .expect("Failed to read line");

    client_to_send = client_to_send_untrim.trim().into();

    let mut amount = String::new();
    let parsed_amount: u32;

    loop {
        println!("how much would you like to send?");
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");

        parsed_amount = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input");
                amount.clear();
                continue;
            },
        };
        break;
    }

    let mut client = BitcoinClient::connect(SERVER).await?;
    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: String::from(client_name),
        to_addr: String::from(&client_to_send), 
        amount: parsed_amount,
    });

    let response = client.send_payment(request).await?;

    let resp_unwrap = response.into_inner();
    let message = resp_unwrap.message;
    let success = resp_unwrap.successful;

    if success {
        println!("payment complete: {} -> {}", String::from(client_name), client_to_send);
    } else {
        println!("payment failed: {}", message);
    }
    Ok(())
}

async fn sign_in(client_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(SERVER).await?;
    let request = tonic::Request::new(BtcSignIn {
        client_id: String::from(client_name),
    });

    let response = client.open_connection(request).await?;
    let resp = response.into_inner();
    let success = resp.successful;
    let message = resp.message;
    if !success {
        panic!("logged in on another device");
    } else {
        println!("{}", message);
    }
    Ok(())
}

async fn close_con(client_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(SERVER).await?;
    let request = tonic::Request::new(BtcExitInit {
        client_id: String::from(client_name),
    });

    client.close_connection(request).await?;

    Ok(())
}

async fn get_balance(client_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(SERVER).await?;
    let request = tonic::Request::new(BtcBalanceRequest {
        client_id: String::from(client_name),
    });

    // definition in server.rs
    let response = client.check_balance(request).await?;

    println!("you have {} bitcoins", response.into_inner().balance);

    Ok(())
}
