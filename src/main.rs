use ethers::prelude::*;
use eyre::Result;
use std::{fs, str::FromStr, sync::Arc};
use wager_cli::prelude::*;
extern crate dotenv;
use ethers::utils::parse_ether;
use std::fs::File;
use std::io::Write;

use clap::{arg, Command};

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    players: Vec<String>,
}

// struct Instance {
//     contract_address: Address,
//     players_address: Vec<Address>,
//     // wager: Wager<M>,
//     // instance: Wager<SignerMiddleware<Provider<Provider>, Wallet<SigningKey>>>,
// }

fn cli() -> Command {
    Command::new("wager_cli")
        .about("A basic CLi for interacting with Wager contract")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("add")
                .about("adds players to the wager")
                .arg_required_else_help(true)
                .arg(
                    arg!(<PLAYERS> ... "Players to add").value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("show")
                .about("show players")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("remove")
                .about("delete players")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("balance")
                .about("show balances")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("set_bet_amount")
                .about("set bet amount")
                .arg(arg!(<AMOUNT> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("provide_outcome")
                .about("provide outcome of the wager")
                .arg(arg!(<OUTCOME> "Outcome for the wager"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load variables from .env file
    dotenv::dotenv().ok();

    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let contract_address = std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");

    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet: LocalWallet = private_key.parse()?;

    let signer_middleware = SignerMiddleware::new(provider, wallet);

    let client = Arc::new(signer_middleware);

    let wager = Wager::new(
        client,
        Address::from_str(contract_address.as_str()).expect("invalid address"),
    );

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let players = sub_matches
                .get_many::<String>("PLAYERS")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Adding {players:?}");
            let mut players_db: Vec<String> = Vec::new();
            let mut players_addr: Vec<Address> = Vec::new();

            for i in players {
                players_db.push(i.to_owned());
                players_addr
                    .push(Address::from_str(i.to_owned().as_str()).expect("not a valid address"));
            }

            let data = Data {
                players: players_db,
            };

            store_as_json(data);

            wager.add_players(players_addr).await?;
        }
        Some(("show", _)) => match read_from_json() {
            Some(players) => println!("players: {:?}", players),
            None => println!("Players have not been added yet"),
        },
        Some(("remove", _)) => {
            match read_from_json() {
                Some(players) => {
                    let mut players_addr: Vec<Address> = Vec::new();
                    for i in players.players {
                        players_addr.push(Address::from_str(&i).expect("Invalid address"));
                    }
                    wager.remove_players(players_addr).await?;
                    fs::remove_file("players.json").expect("file does not exist yet");
                }
                None => println!("Players have not been added yet"),
            }
            // fs::remove_file("players.json").expect("file does not exist yet");
            println!("deleting players");
        }
        Some(("balance", _)) => {
            println!("contract balance: {:?}", wager.balance().await?);
        }
        Some(("set_bet_amount", sub_matches)) => {
            let amount = sub_matches.get_one::<String>("AMOUNT").expect("required");

            let amount_in_wei: U256 =
                parse_ether(U256::from_str(amount).expect("invalid U256")).unwrap();

            wager.set_bet_amount(amount_in_wei).await?;

            println!("setting bet amount to {amount_in_wei}");
        }
        Some(("provide_outcome", sub_matches)) => {
            let outcome = sub_matches.get_one::<String>("OUTCOME").expect("required");

            // Convert the string to H256
            let hash: H256 = outcome.parse().expect("Failed to parse hex string");

            // wager.provide_outcome(hash.into()).await?;

            println!("setting outcome to {hash}");
        }
        _ => {
            println!("invalid command");
            unreachable!()
        } // If all subcommands are defined above, anything else is unreachable!()
    }

    Ok(())
}

fn store_as_json(data: Data) {
    // Serialize the struct to a JSON string
    let json_string = serde_json::to_string(&data).expect("Failed to serialize to JSON");

    // Open a file for writing
    let mut file = File::create("players.json").expect("Failed to create file");

    // Write the JSON string to the file
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");

    println!("storing players in json");
}

fn read_from_json() -> Option<Data> {
    match std::fs::read_to_string("players.json") {
        Ok(p) => {
            return Some(serde_json::from_str(&p).expect("Error parsing JSON"));
        }
        Err(_) => return None,
    }
}
