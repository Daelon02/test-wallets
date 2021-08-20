use bip39::{Language, Mnemonic, MnemonicType};
use log::{error, info, trace, warn};
use postgres::NoTls;
use primitives::{Config, Raw, Request, ResponderRequest, Error};
use sp_core::{
    crypto::{AccountId32, Ss58Codec},
    Pair,
};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_postgres::Client;
use web3::api::{ParityAccounts, Namespace};
use utils::*;
use utils::contract::connect;
use web3::types::Address;
use web3::transports::WebSocket;

#[tokio::main]
async fn main() {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase = mnemonic.phrase();

    println!("phrase: {}", phrase);

    let url = "wss://data-seed-prebsc-1-s1.binance.org:8545";
    // Connect to bsc
    let mut wss = WebSocket::new(url).await;

    let transport = ParityAccounts::new(wss.unwrap());

    let address =
        ParityAccounts::parity_new_account_from_phrase(&transport, phrase, "").await;
    // println!("account_id: {:?}", pair.to_string());
    println!("account_id: {:?}", address);

}