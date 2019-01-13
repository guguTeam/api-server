extern crate websocket;
extern crate rmp;
extern crate bcrypt;
extern crate timer;
extern crate chrono;

mod protocol;
mod server;

fn main() {
    server::run("0.0.0.0", 19132);
}