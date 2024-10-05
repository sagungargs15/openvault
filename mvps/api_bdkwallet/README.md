# Learning Objectives
1. AXUM APIs
2. BDK Bitcoin Wallet
3. Small Rust Functions
    (i) AXUM Swagger
    (ii) Reading from a File
    (iii) Basic Aith Login

# How to Run on your local machine

## Open Terminal 
> cd api_bdkwallet 
> cargo build
> cargo run

## Open Browser

TODO:Check if the Rust crate "api_bdkwallet" successfuly ran by opening your Browser: http://127.0.0.1:8080 and seeing "Hello World"

TODO: Check if the Swagger UI - OPEN API specs for the two created APIs are visible: http://127.0.0.1:8080/redoc

You are currently on Bitcoin testnet. You end up generating a new Wallet via BDK by trigerring the AXUM API endpoint
Hit URL http://127.0.0.1:8080/api/gen_wallet in your Web Browser on your Local machine (Every time you run this endpoint a new wallet address is created using the same "xprv" loaded from wallet_config.json )
Copy Address Generated on HTML Page 
e.g. Page shows: {"amount":null,"message":"tb1qlxrj38nf6h469xhnh97s80m5nes4xspf3t949h"}
The new Bitcoin Wallet XPUB address generated on Bitcoin testnet is "tb1qlxrj38nf6h469xhnh97s80m5nes4xspf3t949h"
Now open Bitcoin testnet Faucet URL: https://bitcoinfaucet.uo1.net/. Use this platform to receive some dummy Bitcoin Sats on testnet on the above Wallet address i.e "tb1qlxrj38nf6h469xhnh97s80m5nes4xspf3t949h"
Once you hit receiving the dummy Bitcoin on testnet via https://bitcoinfaucet.uo1.net/ and using your newly generated wallet address: "tb1qlxrj38nf6h469xhnh97s80m5nes4xspf3t949h" you will see the below screen
![Image](/api_bdkwallet/SuccessfulReceiveDummyBitcoinOnTestnet.png)


## We have 3 API routes designed

1. http://127.0.0.1:8080/api/login
```
curl -X POST http://127.0.0.1:8080/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "demo1", "password": "welcome"}'
```
Response:
``` 
{"result":{"success":true}}
```

2. http://127.0.0.1:8080/api/gen_wallet
```
curl "http://127.0.0.1:8080/api/gen_wallet"
```
Response:
```
{"amount":null,"message":"tb1qf9jzn8lhy9yrnm06p2cwzjhr70e2hh957mxqzf","xprv":"tprv8ZgxMBicQKsPdq5jcapJibW4rBwjKn6bdtgHXR4Fj8zPbzH2X82ivPfot44yq1Mj5GTHfH1inGDMUWUpXk1r23q1KyesPmHiA4T74jmokFV"}


3. http://127.0.0.1:8080/api/load_wallet
```
curl "http://127.0.0.1:8080/api/load_wallet"
```
Response:
```
{"amount":null,"message":"tb1qgym6c77ch0juwrlwv2yck6mp0wqtkhde4ppum8"}
```


# Learning Rust Axum BDK

My bad..I thought BDK needed to use a database...Duh! - That's what the xprv is for!
This code will look ugly, I'll refactor in due course.

BIP44 specifies the structure as consisting of five
predefined tree levels:
    m / purpose' / coin_type' / account' / change / address_index

https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch05.asciidoc#extended-keys

uses : Bitcoin Testnet + Electrum

- Create Bitcoin Wallet and save xrpv to json (totally insecure, but this is for learning)
- Load wallet from xprv
- Create PSBT + Sign + Broadcast

  Note : tb1 = testnet bech32 addresses / corresponding prefix on mainnet would be bc1

#### https://redandgreen.co.uk/bitcoin-testnet-test-faucet/bitcoin-programming/

#### endpoints
    /
    api/gen_wallet
    api/load_wallet


#### xprv is read from json
    {"xprv":"xprv9s21ZrQH143K2x5hVoEpv7wE9e4Mk4eeMW2yj1P6EYsL3bexhFi4aFKMWeWw5p7u3kGtZonxoX3EPp3oQtBsxSmWqw6TcTq6ttNt44QhXDs","network": "testnet"}
