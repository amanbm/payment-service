<img width="845" alt="Screen Shot 2022-10-12 at 4 27 31 PM" src="https://user-images.githubusercontent.com/19520861/195466641-9d355763-1b0b-420b-ba2f-058f3bd19880.png">

# Simple gRPC based Payment Service

## Basic use
Clients can log in and send money to eachother via a command line interface:


The server logs user activity: 

<img width="173" alt="Screen Shot 2022-10-12 at 4 30 32 PM" src="https://user-images.githubusercontent.com/19520861/195466769-061facdd-c61f-4832-87dd-90d111adc90a.png">

The server supports atomic transactions with multiple live clients.

<img width="283" alt="Screen Shot 2022-10-12 at 4 30 14 PM" src="https://user-images.githubusercontent.com/19520861/195466864-7720d2e0-cf5d-4c67-9423-62f53185aed0.png">



Run payment server
```
cargo run --bin payments-server
```

Run payment client
```
cargo run --bin payments-client
```
