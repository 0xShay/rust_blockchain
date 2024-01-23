use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::thread;
use reqwest::blocking::Response;
use local_ip_address::local_ip;
use crate::peers::Peers;

pub struct Server {
    peers_object: Peers
}

const port: u16 = 7000; // there are 2^16 - 1 different network ports, all unsigned integers
// TODO: allow the port to be changed by client/wallet

impl Server {
    pub fn new() -> Self{
        Server {
            peers_object: Peers::new()
        }
    }
    pub fn start_server(&mut self) {
        self.peers_object.load_known_peers();
        // self.peers_object.update_known_peers();
        // TODO: get another client or two to test updating known peers
        // println!("Known peers after updating: {:#?}", self.peers_object);

        let listener = TcpListener::bind(format!("{}:{}", local_ip().unwrap(), port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_request(stream);
        }
    }

    pub fn stop_server(&self) {
        self.peers_object.save_known_peers();
    }

    pub fn handle_request(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
        println!("IP Address of requester: {}", stream.peer_addr().unwrap());

        if http_request.get(0).is_none() {
            self.peers_object.remove_ip(stream.peer_addr().unwrap());
        }
        else {
            match String::from(&http_request.get(0).unwrap()[..4]).as_str() {
                "GET " => {self.handle_get(stream)}
                "POST" => {self.handle_post(stream)}
                _ => {self.peers_object.remove_ip(stream.peer_addr().unwrap())}
            }
        }

    }
    
    pub fn handle_get(&mut self, mut stream: TcpStream) {
        let response = serde_json::to_string(&self.peers_object.get_known_peers(stream.local_addr().unwrap())).expect("Could not serialise known peers array.");
        let response = serde_json::to_string(&self.peers_object.get_known_peers(stream.peer_addr().unwrap())).expect("Could not serialise known peers array.");
        let response_str = format!("HTTP/1.1 200 OK\r\n\r\n{}", response);
        stream.write_all(response_str.as_bytes()).unwrap();
    }

    pub fn handle_post(&mut self, mut stream: TcpStream) {
        // broadcast has occurred, TODO: implement adding block
    }

    pub fn send_get_request(addr_to: SocketAddr) -> reqwest::Result<Response> {
        reqwest::blocking::Client::new().get(format!("http://{addr_to}/")).header("port", port).send()
    }

    pub fn send_post_request(addr_to: IpAddr) {

    }
}
