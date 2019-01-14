extern crate websocket;
extern crate bcrypt;
extern crate lazy_static;
extern crate config;
extern crate log;
extern crate simplelog;

use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::collections::HashMap;
use std::io::Write;

use config::Config;

use simplelog::{CombinedLogger, TermLogger, LevelFilter, Config as LogConfig, Level};

use lazy_static::lazy_static;

use crate::protocol::Packet;

mod stream;
mod protocol;
mod server;

lazy_static!{
    static ref CONFIG: Mutex<Config> = Mutex::new({
        let mut config = Config::default();
        let isnew = Path::new("config.toml").exists();
        if !isnew {
            OpenOptions::new().write(true).create(true).open("config.toml").unwrap().write_all(include_bytes!("../config.toml")).unwrap();
        }
        config.merge(config::File::with_name("config")).unwrap();
        config
    });
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, LogConfig {
                time: Some(Level::Error),
                level: Some(Level::Error),
                target: Some(Level::Debug),
                location: Some(Level::Debug),
                time_format: Some("%m/%d %:z %H:%M:%S"),
            }).unwrap()
        ]
    ).unwrap();
    log::info!("Logger initialization completed");
    let xx = Vec::from("我是密码1234");
    let p: &[u8] = &xx;
    let mut pk = protocol::authorization_packet::new(p.to_vec());
    pk.encode();
    //println!("{:?}", pk.password);
    let config = CONFIG.lock().unwrap().clone();
    server::run(config.get::<String>("listen.ip").unwrap().as_str(), config.get::<i32>("listen.port").unwrap());
}
