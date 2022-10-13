use payments::bitcoin_client::BitcoinClient;
use payments::{
    BtcBalanceRequest, BtcSignIn, LiveUsersRequest, BtcPaymentRequest, BtcExitInit,
};
use std::io;
use std::env;
use figlet_rs::FIGfont;

const EXIT: u32 = 4;
const SERVER: &str = "http://172.20.10.8:50052";


pub mod payments {
    tonic::include_proto!("payments");
}

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
            _ => println!("try again"),
        }
        println!("\n");
    }
    Ok(())
}

async fn all_users(client_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(SERVER).await;
    let mut client_res = loop {
        match client {
            Ok(res) => break res,
            Err(_) => {
                println!("Server seems to be down. Hit enter to retry, or Ctrl-c to exit");
                let mut ret = String::new();
                io::stdin()
                    .read_line(&mut ret)
                    .expect("Failed to read line");
                client = BitcoinClient::connect(SERVER).await;
            }
        }
    };

    let request = tonic::Request::new(LiveUsersRequest {
        client_id: String::from(client_name),
    });

    let response = client_res.get_users(request).await?.into_inner();
    let users_str = response.users;
    let len = users_str.len();
    println!("{}", 
        if len == 0 {
            String::from("No other users at this time")
        } else {
            String::from(users_str)
        });
    Ok(())
}

async fn send_payment(client_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client_to_send_untrim = String::new();
    let mut client_to_send = String::new();
    println!("who would you like to send coins to?");

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
    // check if connection can be made TODO: Refactor
    let mut client = BitcoinClient::connect(SERVER).await;
    let mut client_res = loop {
        match client {
            Ok(res) => break res,
            Err(_) => {
                println!("Server seems to be down. Hit enter to retry, or Ctrl-c to exit");
                let mut ret = String::new();
                io::stdin()
                    .read_line(&mut ret)
                    .expect("Failed to read line");
                client = BitcoinClient::connect(SERVER).await;
            }
        }
    };

    let request = tonic::Request::new(BtcSignIn {
        client_id: String::from(client_name),
    });

    let response = client_res.open_connection(request).await?;
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

    println!("you have {} coins", response.into_inner().balance);

    Ok(())
}
