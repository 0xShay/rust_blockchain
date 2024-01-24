use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct Peers {
    known_peers: Vec<SocketAddr>
}

impl Peers {

    pub fn new() -> Self {
        let mut peers: Peers = Peers {
            known_peers: Vec::new()
        };
        println!("Loading known peers from file...");
        peers.load_known_peers();
        peers
    }

    pub fn from_json(json: &str) -> Vec<SocketAddr> {
        serde_json::from_str(json).expect("Error deserializing peers JSON")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.known_peers).expect("Error serializing peers JSON")
    }

    pub fn get_known_peers(&mut self) -> Vec<SocketAddr> {
        self.known_peers.clone()
    }

    pub async fn update_known_peers(&mut self) {
        let mut new_known_peers: Vec<SocketAddr> = Vec::new();
        let known_peers_copy = self.known_peers.clone();
        let client_builder = reqwest::ClientBuilder::new().timeout(Duration::new(5, 0));
        let client: reqwest::Client = client_builder.build().expect("Building client failed");
        for peer in &known_peers_copy {
            println!("{}", peer);
            match client.get(format!("http://{}/peers", peer)).send().await {
                Ok(mut response) => {
                    // check if 200 OK
                    if response.status() == reqwest::StatusCode::OK {
                        println!("Request to peer {} OK", peer);
                    } else {
                        println!("Request to peer {} not OK", peer);
                    }
                },
                Err(err) => {
                    println!("Request to peer {} failed, {:?}", peer, err);
                }
            }
        }
        self.known_peers = new_known_peers;
        self.save_known_peers();
    }

    pub fn save_known_peers(&self) {
        let path = Path::new("known_peers.txt");
        let mut file = match File::create(&path) {
            Ok(file) => {file}
            Err(err) => {panic!("Could not create file called {}: {}", path.display(), err)}
        };
        for known_peer in &self.known_peers {
            let mut str = known_peer.to_string();
            str.push('\n');
            file.write_all(str.as_bytes()).expect("Could not write to the file");
        }
    }

    pub fn load_known_peers(&mut self) {
        let path = Path::new("known_peers.txt");
        let mut file = match File::open(&path) {
            Ok(file) => {file}
            Err(err) => {panic!("Could not open file called {}: {}", path.display(), err)}
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read contents of file");
        self.known_peers = Vec::new();
        for line in contents.split("\n") {
            if line.is_empty() { continue }
            self.known_peers.push(line.parse().expect("Could not convert to type IPAddr"));
        }
    }

    pub fn remove_ip(&mut self, ip_addr: SocketAddr) {
        self.known_peers.retain(|&x| x != ip_addr);
        self.save_known_peers();
    }

}
