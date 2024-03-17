#[macro_use] extern crate rocket;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rocket::http::Status;
use rocket::response::{content, status};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use tokio;

mod block_utils;
use block_utils::Block;

mod account_utils;
mod peers;
mod db_utils;

#[get("/block/<hash>")]
fn get_block(hash: &str) -> status::Custom<content::RawJson<String>> {
    match db_utils::get_block(hash) {
        Some(block) => {
            let block_json: String = block.to_json();
            status::Custom(Status::Accepted, content::RawJson(block_json))            
        }
        None => {
            status::Custom(Status::NotFound, content::RawJson(String::from("{
                \"error\": \"A block with the given hash was not found in the database.\"
            }")))
        }
    }
}

#[get("/frontier_block")]
fn get_frontier_block() -> status::Custom<content::RawJson<String>> {
    match db_utils::get_frontier_block() {
        Some(block) => {
            let block_json: String = block.to_json();
            status::Custom(Status::Accepted, content::RawJson(block_json))            
        }
        None => {
            status::Custom(Status::NotFound, content::RawJson(String::from("{
                \"error\": \"An error occurred querying the database.\"
            }")))
        }
    }
}

#[get("/peers")]
fn get_peers(remote_addr: SocketAddr) -> status::Custom<content::RawJson<String>> {
    let mut peers = peers::Peers::new();
    status::Custom(Status::Accepted, content::RawJson(peers.to_json()))            
}

#[post("/publish_block", format="application/json", data="<block_json>")]
fn post_block(block_json: String) -> status::Custom<content::RawJson<String>> {
    match Block::from_json(&block_json) {
        Ok(block) => {
            if Block::verify_block_is_valid(&block) {
                db_utils::add_block(Block::add_acc_diff_to_block(block));
                status::Custom(Status::Accepted, content::RawJson(block_json))
            } else {
                status::Custom(Status::BadRequest, content::RawJson(String::from("{
                    \"error\": \"Block verification failed.\"
                }")))
            }
        },
        Err(err) => {
            println!("{:?}", err);
            status::Custom(Status::BadRequest, content::RawJson(String::from("{
                \"error\": \"The supplied block JSON was malformed.\"
            }")))
        }
    }
}

#[launch]
fn rocket() -> _ {

    db_utils::create_tables();

    let mut peers = peers::Peers::new();

    println!("Known peers: {:#?}", peers);

    println!("Saving to known_peers.txt...");
    peers.save_known_peers();

    println!("Updating known peers");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            peers.update_known_peers().await;
        });

    println!("---");
    println!("Generating genesis block...");

    let genesis: Block = Block::generate_genesis_block();
    println!("{:#?}", genesis);
    println!("{}", genesis.to_json());

    db_utils::add_block(genesis);

    // check peers to see if frontier block index > local block index,
        // if so, fetch blocks sequentially and verify each block before appending to db

    rocket::build().mount("/", routes![
        get_block,
        get_peers,
        get_frontier_block,
        post_block
    ])

}

    // println!("---");
    // println!("Generating keys...");

    // let priv_key: SigningKey = account_utils::generate_priv_key();
    // let priv_key_hex: String = account_utils::priv_key_to_hex(&priv_key);
    
    // let pub_key: VerifyingKey = account_utils::generate_pub_key(&priv_key);
    // let pub_key_hex: String = account_utils::pub_key_to_hex(&pub_key);

    // println!("Generated private key: {}", priv_key_hex);
    // println!("Generated public key: {}", pub_key_hex);


    // println!("---");
    // println!("Generating genesis block...");

    // let genesis: Block = Block::generate_genesis_block();
    // println!("{:#?}", genesis);
    // println!("{}", genesis.to_json());
    
    // println!("Getting frontier block...");
    // let frontier: Block = db_utils::get_frontier_block().expect("No frontier block");
    // println!("{:#?}", frontier);

    // return;

    // Test peers.rs code