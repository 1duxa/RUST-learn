use std::process::exit;
use std::vec;

use bitcoincash_addr::Address;
use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;
use crate::wallet::Wallets;

pub struct Cli {}
impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("Blockchain rust")
            .version("0.1")
            .author("andrewvezdel@gmail.com")
            .about("doing blockchain")
            .subcommand(Command::new("printchain").about("print all chain blocks"))
            .subcommand(
                Command::new("getbalance")
                    .about("get balance")
                    .arg(arg!(<ADDRESS>"'addres it get balance for'")),
            )
            .subcommand(
                Command::new("create")
                    .about("Create blockchain")
                    .arg(arg!(<ADDRESS>"'the addres to send genesis block to'")),
            )
            .subcommand(
                Command::new("send")
                    .about("send in the blockchain")
                    .arg(arg!(<FROM>"'source wallet address'"))
                    .arg(arg!(<TO>"'destination wallet address'"))
                    .arg(arg!(<AMOUNT>"'amount of racks'")),
            )
            .subcommand(Command::new("createwallet").about("Create a wallet"))
            .subcommand(Command::new("listaddresses").about("list all addresses"))
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("created blockchina")
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let pub_key_hash = Address::decode(&address).unwrap().body;
                let bc= Blockchain::new()?;
                let utxos = bc.find_UTXO();
                let mut balance = 0;
                for out in utxos {
                    for out2 in out.1.outputs{ 
                    balance += out2.value;
                }}
                println!("balance of {} is {}", address, balance);
            }
        }
        if let Some(_) = matches.subcommand_matches("printchain") {
            Self::cmd_print_chain()?;
        }
        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("there is no FROM supplied");
                exit(1)
            };
            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("there is no TO supplied");
                exit(1)
            };
            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("there is no AMOUNT supplied");
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;

            bc.push_block(vec![tx])?;
            println!("SUCCESS! :DDDDDDD XDDDDD :PP");
        }
        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("Addres: {}",address);
        }
        if let Some(_) = matches.subcommand_matches("listaddresses") {
            let ws = Wallets::new()?;
            let address = ws.get_all_addresses();
            
            println!("Addresses");

            for adr in address {
                println!("| {} |",adr);
            }
        }
        Ok(())
    }
    fn cmd_print_chain() -> Result<()> {
        let bc = Blockchain::new()?;
        for b in bc.iter() {
            println!("{:#?}", b);
        }
        Ok(())
    }
}
