mod error;
pub mod ffi;
#[allow(dead_code)]
mod message;

use crate::error::Error;
use crate::message::{Attribute, Class, Message, Method};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

///
/// No need for async runtime (Tokio or async-std) as this library is intended to be used
/// at a very low frequency (i.e.: when client wants to place or respond to a call)
///
pub fn get_xor_mapped_address(
    stun_address: SocketAddr,
    local_port: &str,
    options: Options,
) -> Result<SocketAddr, Error> {
    let (request_bytes, transaction_id) = build_payload(options.software);
    let socket = post_stun_request(request_bytes, stun_address, local_port)?;
    get_stun_result(socket, options.timeout, transaction_id)
}

fn post_stun_request(
    bytes: Vec<u8>,
    stun_address: SocketAddr,
    local_port: &str,
) -> Result<UdpSocket, Error> {
    // TODO : check if 0.0.0.0 works with all setup (127.0.0.1 does not work on macOS)
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", local_port)).map_err(|err| {
        println!("Could not bind: {}", err);
        Error::DefaultError(format!("{}", err))
    })?;
    socket.connect(stun_address).map_err(|err| {
        println!("Could not connect: {}", err);
        Error::DefaultError(format!("{}", err))
    })?;
    socket.send(&bytes.as_slice()).map_err(|err| {
        println!("Could not send bytes: {}", err);
        Error::DefaultError(format!("{}", err))
    })?;
    #[cfg(debug_assertions)]
    println!("Sent!!");
    Ok(socket)
}

fn get_stun_result(
    socket: UdpSocket,
    timeout: Duration,
    transaction_id: Vec<u8>,
) -> Result<SocketAddr, Error> {
    let mut buf = [0; 256];
    socket.set_read_timeout(Some(timeout)).map_err(|err| {
        println!("Could not set timeout : {}", err);
        Error::DefaultError(format!("{}", err))
    })?;
    socket.recv_from(&mut buf).map_err(|err| {
        println!("Could not receive : {}", err);
        Error::DefaultError(format!("{}", err))
    })?;
    let message = Message::from_raw(&buf).map_err(|err| Error::DefaultError(format!("{}", err)))?;
    if message.get_transaction_id() != transaction_id {
        return Err(Error::DefaultError("Invalid transaction id".to_string()));
    }
    let address =
        Attribute::get_xor_mapped_address(&message).expect("Could not decode xor mapped address");
    Ok(address)
}

fn build_payload(software: Option<String>) -> (Vec<u8>, Vec<u8>) {
    let mut attributes = None;
    if let Some(software) = software {
        let mut attribute_map = HashMap::<Attribute, Vec<u8>>::new();
        attribute_map.insert(Attribute::Software, software.as_bytes().to_vec());
        attributes = Some(attribute_map);
    }
    let message = Message::new(Method::Binding, Class::Request, attributes);
    (message.to_raw(), message.get_transaction_id())
}

pub struct Options {
    timeout: Duration,
    software: Option<String>,
}

impl Options {
    pub fn new(timeout: Option<Duration>, software: Option<String>) -> Options {
        if let Some(timeout) = timeout {
            return Options { timeout, software };
        }
        Options {
            timeout: Duration::new(10, 0),
            software,
        }
    }
}

#[cfg(test)]
mod test {
    use std::net::{SocketAddr, ToSocketAddrs};

    use crate::{get_xor_mapped_address, Options};

    #[test]
    fn test() {
        let socket_address: SocketAddr = "plato-test.mantoux.org:3478"
            .to_socket_addrs()
            .unwrap()
            .last()
            .expect("Could not parse STUN server");
        let result = get_xor_mapped_address(socket_address, "3522", Options::new(None, None))
            .expect("Error while getting mapped address");
        println!("Mapped address: {}", result)
    }
}
