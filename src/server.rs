use std::thread;

use websocket::sync::Server;
use websocket::OwnedMessage;

use bcrypt::verify;

use log::{info, warn, debug};

use crate::protocol;
use crate::protocol::Packet;

pub fn run(ip: &str, port: i32) {
    let server = Server::bind(format!("{}:{}", ip, port)).unwrap();
    info!("listen on {}:{}", ip, port);
    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let client = request.accept().unwrap();
            let addr = client.peer_addr().unwrap();
            info!("Client {} connection", addr);

            let (mut receiver, mut sender) = client.split().unwrap();

            if let OwnedMessage::Binary(b) = receiver.recv_message().unwrap() {
                let mut pk = protocol::authorization_packet::new(b);
                pk.encode();
                if !verify(super::CONFIG.lock().unwrap().get::<String>("auth.passwd").unwrap().as_str(), pk.password.as_str()).unwrap() {
                    warn!("{}. Login failed", addr);
                    sender.send_message(&OwnedMessage::Close(None)).unwrap();
                    return;
                }
                info!("{}. Login successfully!", addr);
            } else {
                warn!("{}. No binary data was sent!", addr);
                sender.send_message(&OwnedMessage::Close(None)).unwrap();
                return;
            }

            for message in receiver.incoming_messages() {
                match message.unwrap() {
                    OwnedMessage::Close(_) => {
                        sender.send_message(&OwnedMessage::Close(None)).unwrap();
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        sender.send_message(&OwnedMessage::Pong(ping)).unwrap();
                    }
                    OwnedMessage::Text(text) => {
                        warn!("{}. No binary data was sent!\nWhat he sent was {}", addr, text);
                        sender.send_message(&OwnedMessage::Close(None)).unwrap();
                        return;
                    }
                    OwnedMessage::Binary(b) => {

                    }
                    OwnedMessage::Pong(pong) => {
                        debug!("{}. Send a Pong {:?}", addr, pong);
                    }
                }
            }
        });
    }
}
