Configure an Identity:

```
stellar keys generate --global piotrek --network testnet
```

Get the address:

```
stellar keys address piotrek
```

# Deployment

Build the contract:

```
stellar contract build
```

Deploy the contract:

```
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/nfts_creator.wasm \
  --source piotrek \
  --network testnet
```

Initialize the contract:

```
stellar contract invoke \                               
  --id CC7STMVOXVHTJRH5KJC2YOWXMJLSHDPBD5HMJYMDY2RK54BMPNY3WIOQ \
  --source piotrek \
  --network testnet \
  -- \
  initialize
```

# Minting

```
stellar contract invoke \                               
  --id CC7STMVOXVHTJRH5KJC2YOWXMJLSHDPBD5HMJYMDY2RK54BMPNY3WIOQ \
  --source piotrek \
  --network testnet \
  -- mint --uri test --to piotrek --name test --description test
```

# Getting the total number of minted NFTs

```
stellar contract invoke \
  --id CC7STMVOXVHTJRH5KJC2YOWXMJLSHDPBD5HMJYMDY2RK54BMPNY3WIOQ \
  --source piotrek \
  --network testnet \
  -- total_nfts
```
