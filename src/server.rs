use tonic::{transport::Server, Request, Response, Status};

use crate::user_info::User;
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{
    BtcBalanceRequest, BtcBalanceResponse, BtcExitAck, BtcExitInit, BtcPaymentRequest,
    BtcPaymentResponse, BtcSignIn, BtcSignInAck,
};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, User>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

pub mod payments {
    tonic::include_proto!("payments");
}

mod user_info;

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply;
        let req = request.into_inner();
        let tx = req.from_addr;
        let rx = req.to_addr;
        let amount_to_send: u32 = req.amount;

        let mut map = HASHMAP.lock().unwrap();

        // sending money to self is not allowed
        if tx == rx {
            println!("sender == receiver");
            reply = BtcPaymentResponse {
                successful: false,
                message: format!("sender == receiver").into(),
            };
            return Ok(Response::new(reply))
        }

        // make sure receiver is a valid client
        if !map.contains_key(&rx) {
            println!("{}: unknown receiver", tx);
            reply = BtcPaymentResponse {
                successful: false,
                message: format!("unkown receiver").into(),
            };
            return Ok(Response::new(reply))
        }

        let tx_current_balance = map.get(&tx).unwrap().balance;
        let rx_current_balance = map.get(&rx).unwrap().balance;
        let tx_access = map.get(&tx).unwrap().access;
        let tx_live = map.get(&tx).unwrap().live;
        let rx_access = map.get(&rx).unwrap().access;
        let rx_live = map.get(&rx).unwrap().live;

        // check that current balance of sender is enough to handle request
        if tx_current_balance < amount_to_send as i32 {
            println!("{}: insufficient funds", tx);
            reply = BtcPaymentResponse {
                successful: false,
                message: format!("insufficient funds").into(),
            };
            return Ok(Response::new(reply))
        }

        // update sender and receiver with correct balances
        let user_tx = User {
            client_id: String::from(&tx),
            balance: (tx_current_balance - amount_to_send as i32),
            access: tx_access,
            live: tx_live,
        };
        let user_rx = User {
            client_id: String::from(&rx),
            balance: rx_current_balance + amount_to_send as i32,
            access: rx_access,
            live:  rx_live,
        };
        map.insert(tx, user_tx);
        map.insert(rx, user_rx);

        reply = BtcPaymentResponse {
            successful: true,
            message: format!("complete").into(),
        };

        Ok(Response::new(reply))
    }

    async fn check_balance(
        &self,
        request: Request<BtcBalanceRequest>,
    ) -> Result<Response<BtcBalanceResponse>, Status> {
        let req = request.into_inner();
        println!("{} checking balance", req.client_id);
        let map = HASHMAP.lock().unwrap();
        let balance = map.get(&req.client_id).unwrap().balance;
        let reply = BtcBalanceResponse { balance: balance };

        Ok(Response::new(reply))
    }

    async fn close_connection(
        &self,
        request: Request<BtcExitInit>,
    ) -> Result<Response<BtcExitAck>, Status> {
        let req = request.into_inner();
        println!("see you next time {}!", req.client_id);

        let mut map = HASHMAP.lock().unwrap();

        let curr_entry = map.get(&req.client_id).unwrap();
        let updated_entry = User {
            client_id: String::from(&req.client_id),
            balance: curr_entry.balance,
            access: curr_entry.access,
            live: false,
        };
        map.insert(req.client_id, updated_entry);
        let reply = BtcExitAck {};
        Ok(Response::new(reply))
    }

    async fn open_connection(
        &self,
        request: Request<BtcSignIn>,
    ) -> Result<Response<BtcSignInAck>, Status> {
        let mut resp_message = String::new();
        let req = request.into_inner();
        let mut map = HASHMAP.lock().unwrap();

        let updated_entry;

        // never seen client before so must not already be online
        if !map.contains_key(&req.client_id) {
            resp_message = format!("Welcome new user {}!", req.client_id);
            println!("Welcome new user {}!", req.client_id);
            updated_entry = User {
                client_id: String::from(&req.client_id),
                balance: 100,
                access: 1,
                live: true,
            };
        } else {
            // prevent clients from logging in twice (i.e. one session per client)
            if map.get(&req.client_id).unwrap().live {
                resp_message = format!("{} already logged in, rejecting connection", req.client_id);
                println!("{} already logged in, rejecting connection", req.client_id);
                let reply = BtcSignInAck {
                    successful: false,
                    message: String::from(resp_message),
                };
                return Ok(Response::new(reply))
            }
            resp_message = format!("Welcome back {}", req.client_id);
            println!("Welcome back {}", req.client_id);
            let currEntry = map.get(&req.client_id).unwrap();
            updated_entry = User {
                client_id: String::from(&req.client_id),
                balance: currEntry.balance,
                access: currEntry.access,
                live: true,
            };
        }

        let reply = BtcSignInAck {
            successful: true,
            message: String::from(resp_message),
        };
        map.insert(req.client_id, updated_entry);
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "172.20.10.8:50052".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
