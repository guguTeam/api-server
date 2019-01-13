use rmp;

pub struct AuthorizationPacket {
password: String
}

impl AuthorizationPacket {

}

pub fn encode(b: Vec<u8>) -> AuthorizationPacket {
let mut out = [0u8; 16];
AuthorizationPacket{password: String::from(rmp::decode::read_str(&mut b, &mut out).unwrap())};
}