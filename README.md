# Web3 Anywhere

## Introduction

Framework utilize graphql to make web3 development easier

## Packages

- `client`: client package includes `crypto`, `key-man` and `near`
- `crypto`: crypto package provides `secp256k1` and `ed25519`
- `key-man`: key management package provides key storage api
- `near`: Near protocol client package provides wallet api

### Commands

- `cargo make format` - Formats the code
- `cargo make clean` - Removes the build directory
- `cargo make build` - Builds the program
- `cargo make test` - Runs the tests

### Add new commands

Please modify the `Makefile.toml` file to add new commands.
