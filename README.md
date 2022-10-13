
<img width="532" alt="Screen Shot 2022-10-12 at 11 48 46 PM" src="https://user-images.githubusercontent.com/19520861/195523596-125bd451-6b74-4800-bfa5-c19b0f65350c.png">

# Simple gRPC based Payment Service

## Basic use
Clients can log in and send money to eachother via a command line interface:


The server logs user activity: 
<img width="173" alt="Screen Shot 2022-10-12 at 4 30 32 PM" src="https://user-images.githubusercontent.com/19520861/195466769-061facdd-c61f-4832-87dd-90d111adc90a.png">

The server supports atomic transactions with multiple live clients.
<img width="283" alt="Screen Shot 2022-10-12 at 4 30 14 PM" src="https://user-images.githubusercontent.com/19520861/195466864-7720d2e0-cf5d-4c67-9423-62f53185aed0.png">

The server keeps track of who is online
<img width="287" alt="Screen Shot 2022-10-12 at 11 54 59 PM" src="https://user-images.githubusercontent.com/19520861/195523813-74ab4819-ff08-4c79-80f6-88beb113e5cd.png">




Run payment server
```
cargo run --bin payments-server
```

Run payment client
```
cargo run --bin payments-client
```
