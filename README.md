# Wager_cli

## Set up environment variables 
The `.env.example` should look like this 
```env 
RPC_URL=
PRIVATE_KEY=
CONTRACT_ADDRESS=
```
To copy it to your `.env` run:
```bash 
cp .env.example .env
```
After the copying the template you should fill in each environmental variable 

## Installing the cli locally 
First you have to clone this repo to your machine:
```bash
git clone https://github.com/Perelyn-sama/wager_cli
```
Next, you have to enter the `wager_cli` then install it 
```bash
cd wager_cli 
cargo install .
```
Now you when you run:
```bash
wager_cli
```
You should see:
```bash
A basic CLi for interacting with Wager contract

Usage: wager_cli <COMMAND>

Commands:
  add              adds players
  show             show players
  remove           delete players
  balance          show balances
  set_bet_amount   set bet amount
  provide_outcome  provide outcome
  submit_bet       submit your bet
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

You can also check out the [Makefile](./makefile) to see how to run commands