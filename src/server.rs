use std::thread;

use websocket::sync::Server;
use websocket::OwnedMessage;

use protocol;

pub fn run(ip: &str, port: i32) {
    let server = Server::bind(format!("{}:{}", ip, port)).unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let client = request.accept().unwrap();
            let addr = client.peer_addr().unwrap();
            println!("Client {} connection", addr);

            let (mut receiver, mut sender) = client.split().unwrap();

if let OwnedMessage::Text(t) = receiver.recv_message().unwrap() {
println!("{}", t);
}

            if let OwnedMessage::Binary(b) = receiver.recv_message().unwrap() {
                let pk = protocol::authorization_packet::encode(b);
                println!("pk: {}", pk.password);
            } else {
                println!("{} closed", addr);
                sender.send_message(&OwnedMessage::Close(None)).unwrap();
                return;
            }

            let message = OwnedMessage::Text("Hello".to_string());
            sender.send_message(&message).unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        return;
                    }

                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }

                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}
