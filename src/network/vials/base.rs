// Relative Modules
pub mod net;

// Standard Uses

// Crate Uses
use crate::network::connection::settings::ConnectionSettings;

// External Uses


pub trait SocketBase {
    fn connected(&self) -> bool;

    fn connect(settings: &ConnectionSettings) -> Result<Self, ()> where Self: Sized;

    fn listen(&mut self);

    fn receive_message(&mut self);

    fn send_bytes(&mut self, data: Vec<u8>);
}


pub(crate) trait SocketBaseBuilder: SocketBase {
    fn new_boxed() {}
}
