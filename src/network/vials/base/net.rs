// Standard Uses
use std::thread;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::{Read, Write};

// Crate Uses
use crate::network::{connection::settings::ConnectionSettings};
use crate::network::packets::packet::{Packet, Receiver};
use crate::network::vials::base::SocketBase;

// External Uses


pub struct Net {
    pub stream: TcpStream,
    pub alive: bool
}

impl SocketBase for Net {

    fn connected(&self) -> bool {
        return self.alive
    }

    fn connect(settings: &ConnectionSettings) -> Result<Self, ()> {
        let address: SocketAddr = format!("{}:{}", settings.ipv4, settings.port).parse().unwrap();
        let timeout = Duration::new(3, 0);

        let stream = TcpStream::connect_timeout(&address, timeout)
            .expect("Could not connect");

        let mut net = Net { stream, alive: false };
        net.listen();

        Ok(net)
    }

    fn listen(&mut self) {
        thread::scope(|_| {
            self.receive_message()
        });
    }

    fn receive_message(&mut self) {
        let mut buffer = vec!();
        let data = self.stream.read_to_end(&mut buffer);

        if data.is_err() {
            println!("we got an exception");
        }

        println!("We got some data: {}", &data.unwrap());
    }

    fn send_bytes(&mut self, data: Vec<u8>) -> () {
        self.stream.write(&*data).unwrap();
    }

}

impl Receiver for Net {
    fn receive_bytes(&self, data: Vec<u8>) {
    }

    fn receive_packet(&mut self, packet: Box<dyn Packet>) -> Result<Box<dyn Packet>, ()> {
        println!("Received packet {}", packet);

        Err(())
    }
}
