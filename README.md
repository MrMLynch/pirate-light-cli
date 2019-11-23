## Pirate Light CLI - A command line Pirate Wallet light client. 

`arrrwallet-cli` is a command line Pirate Wallet light client. To use it, download the latest binary from the releases page and run `./arrrwallet-cli`

This will launch the interactive prompt. Type `help` to get a list of commands

## Running in non-interactive mode:
You can also run `arrrwallet-cli` in non-interactive mode by passing the command you want to run as an argument. For example, `arrrwallet-cli addresses` will list all wallet addresses and exit. 
Run `arrrwallet-cli help` to see a list of all commands. 

## Privacy 
* While all the keys and transaction detection happens on the client, the server can learn what blocks contain your shielded transactions.
* The server also learns other metadata about you like your ip address etc...

## Notes:
* The wallet connects to https://lightd.pirate.black:443 by default. To connect to your own lightd server, please pass `--server https://your_server.com:443`
* If you want to run your own server, please see [pirate lightwalletd](https://github.com/mrmlynch/lightwalletd), and then run `./arrrwallet-cli --server http://127.0.0.1:9068`. You might also need to pass `--dangerous` if you are using a self-signed  TLS certificate.

* The log file is in `~/.pirate/debug-arrrwallet-light-wallet.log`. Wallet is stored in `~/.pirate/arrrwallet-light-wallet.dat`

### Note Management
arrrwallet-cli does automatic note and utxo management, which means it doesn't allow you to manually select which address to send outgoing transactions from. It follows these principles:
* Sapling funds need at least 5 confirmations before they can be spent
* Can select funds from multiple shielded addresses in the same transaction

## Compiling from source

#### Pre-requisites
* Rust v1.37 or higher.
    * Run `rustup update` to get the latest version of Rust if you already have it installed

```
git clone https://github.com/mrmlynch/pirate-light-cli.git
cargo build --release
./target/release/arrrwallet-cli
```

## Options
Here are some CLI arguments you can pass to `arrrwallet-cli`. Please run `arrrwallet-cli --help` for the full list. 

* `--server`: Connect to a custom pirate lightwalletd server. 
    * Example: `./arrrwallet-cli --server 127.0.0.1:9068`
* `--seed`: Restore a wallet from a seed phrase. Note that this will fail if there is an existing wallet. Delete (or move) any existing wallet to restore from the 24-word seed phrase
    * Example: `./arrrwallet-cli --seed "twenty four words seed phrase"`
 * `--recover`: Attempt to recover the seed phrase from a corrupted wallet
 
