extern crate websocket;
extern crate bcrypt;

mod stream;
mod protocol;
mod server;

fn main() {
    server::run("0.0.0.0", 19132);
}