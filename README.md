
<img width="504" alt="Screen Shot 2022-10-12 at 11 56 22 PM" src="https://user-images.githubusercontent.com/19520861/195523959-6df52f6f-30f9-42c3-898f-5f9a65235975.png">

# distributed 'toy' payment service for CLI

# Quick Start Guide
Coming Soon!

# Features
Clients can log in and send money to eachother via a command line interface:

server logs user activity

<img width="173" alt="Screen Shot 2022-10-12 at 4 30 32 PM" src="https://user-images.githubusercontent.com/19520861/195466769-061facdd-c61f-4832-87dd-90d111adc90a.png">

supports atomic transactions with multiple live clients

<img width="283" alt="Screen Shot 2022-10-12 at 4 30 14 PM" src="https://user-images.githubusercontent.com/19520861/195466864-7720d2e0-cf5d-4c67-9423-62f53185aed0.png">

keeps track of who is online

<img width="287" alt="Screen Shot 2022-10-12 at 11 54 59 PM" src="https://user-images.githubusercontent.com/19520861/195523813-74ab4819-ff08-4c79-80f6-88beb113e5cd.png">


Check the cargo.toml file for a list of dependencies

Run payment server
```
cargo run --bin payments-server
```

Run payment client
```
cargo run --bin payments-client
```
