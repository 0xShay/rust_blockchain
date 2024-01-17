use std::fs::File;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct Peers {
    known_peers: Vec<IpAddr>
}

impl Peers {
    pub fn new() -> Self {
        Peers {
            known_peers: Vec::new()
        }
    }
}

impl Peers<> {
    pub fn get_known_peers(&mut self, peer_requested_by: IpAddr) -> Vec<IpAddr> {
        let return_val: Vec<IpAddr> = self.known_peers.clone();
        if !self.known_peers.contains(&&peer_requested_by) {
            // New node has gone online, add it to known peers
            self.known_peers.push(peer_requested_by);
        }
        return_val
    }

    pub fn update_known_peers(&mut self) {
        let new_known_peers: Vec<IpAddr> = self.known_peers.clone(); // TODO: Send request to each known peer
        self.known_peers = new_known_peers;
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
}
