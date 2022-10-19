// client_api
pub mod payments {
    tonic::include_proto!("payments");
}
const SERVER: &str = "http://172.20.10.8:50052";
use payments::bitcoin_client::BitcoinClient;
use payments::{
    BtcBalanceRequest, BtcBalanceResponse, BtcExitInit, BtcPaymentRequest, BtcSignIn,
    LiveUsersRequest,
};
use std::io;
use tokio::runtime::Runtime;
pub fn sync_get_balance(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    Runtime::new().unwrap().block_on(get_balance(&client_name))
}

pub fn sync_sign_in(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    Runtime::new().unwrap().block_on(sign_in(&client_name))
}

pub fn sync_send_payment(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    Runtime::new().unwrap().block_on(send_payment(&client_name))
}

pub fn sync_all_users(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    Runtime::new().unwrap().block_on(all_users(&client_name))
}

pub fn sync_close_con(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    Runtime::new().unwrap().block_on(close_con(&client_name))
}

pub async fn all_users(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
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
    if len == 0 {
        println!("No other users at this time");
        Ok(String::from("No other users at this time"))
    } else {
        println!("{}", String::from(&users_str));
        Ok(String::from(&users_str))
    }
}

pub async fn send_payment(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
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
            }
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
        println!(
            "payment complete: {} -> {}",
            String::from(client_name),
            client_to_send
        );
        Ok(String::from("payment complete"))
    } else {
        println!("payment failed: {}", message);
        Ok(String::from("payment failed"))
    }
}

pub async fn sign_in(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
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
        Ok(String::from(message))
    }
}

pub async fn close_con(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(SERVER).await?;
    let request = tonic::Request::new(BtcExitInit {
        client_id: String::from(client_name),
    });

    client.close_connection(request).await?;

    Ok(String::from("connection closed"))
}

pub async fn get_balance(client_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(SERVER).await?;
    let request = tonic::Request::new(BtcBalanceRequest {
        client_id: String::from(client_name),
    });

    // definition in server.rs
    let response = client.check_balance(request).await?.into_inner();

    println!("you have {} coins", response.balance);

    Ok(response.balance.to_string())
}

