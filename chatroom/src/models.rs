use std::net::SocketAddr;

///  A message is comprise of the contents and the address it was sent on
///   - (String, SocketAddr)
#[derive(Clone, Debug)]
pub struct Message(pub String, pub SocketAddr);
