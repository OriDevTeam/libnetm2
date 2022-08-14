// Relative Modules
pub mod channel;

// Standard Uses

// Crate Uses
use crate::network::connection::settings::ConnectionSettings;
use crate::network::vials::client::channel::ClientChannel;
use crate::network::integrity::synchrony::phase::Phase;

// External Uses


pub struct Client {
    pub channels: Vec<ClientChannel>,
    pub phase: Phase
}

impl Client {

    pub fn new() -> Client {
        Client {channels: vec!(), phase: Phase::Close }
    }

    pub fn connect_channel(&mut self, settings: ConnectionSettings) -> &mut ClientChannel {
        let channel = ClientChannel::connect(settings);
        self.channels.push(channel);

        let channel = self.channels.last_mut().unwrap();

        channel
    }
}

