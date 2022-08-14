// Relative Modules
pub mod net;

// Standard Uses
use std::rc::Rc;

// Crate Uses
use crate::network::connection::settings::ConnectionSettings;
use crate::network::packets::dynamic_manager::DynamicManager;

// External Uses


pub trait SocketBase {
    fn connected(&self) -> bool;

    fn connect(settings: &ConnectionSettings, manager: Rc<DynamicManager>) -> Result<Self, ()> where Self: Sized;
    fn listen(&mut self);

    fn receive_header(&mut self) -> u8;
    fn receive_message(&mut self, header: u8, size: usize) -> Result<Vec<u8>, usize>;

    fn send_bytes(&mut self, data: Vec<u8>);
}


pub(crate) trait SocketBaseBuilder: SocketBase {
    fn new_boxed() {}
}
