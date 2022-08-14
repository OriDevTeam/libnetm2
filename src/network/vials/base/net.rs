// Standard Uses
use std::{io, thread};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::{Read, Write};
use std::rc::Rc;

// Crate Uses
use crate::network::{connection::settings::ConnectionSettings};
use crate::network::packets::dynamic_manager::DynamicManager;
use crate::network::packets::packet::{Packet, Receiver};
use crate::network::vials::base::SocketBase;

// External Uses



pub struct Net {
    pub stream: TcpStream,
    pub alive: bool,
    manager: Rc<DynamicManager>
}


impl SocketBase for Net {

    fn connected(&self) -> bool {
        return self.alive
    }

    fn connect(settings: &ConnectionSettings, manager: Rc<DynamicManager>) -> Result<Net, ()> {
        let address: SocketAddr = format!("{}:{}", settings.ipv4, settings.port).parse().unwrap();
        let timeout = Duration::new(3, 0);

        let stream = TcpStream::connect_timeout(&address, timeout)
            .expect("Could not connect");

        let peer = stream.peer_addr().unwrap();
        println!("Connected to {}:{}", peer.ip(), settings.port);

        let mut net = Net { stream, alive: false, manager };
        net.listen();

        Ok(net)

        // Ok(Net { stream, alive: false })
    }

    fn listen(&mut self) {
        thread::scope(|_| {
            loop {
                let header = self.receive_header();
                let size = self.manager.get_packet_size(&header);

                if size.is_err() {
                    print!("Received packet header {} is not registered", header);
                    continue
                }

                let size = size.unwrap();

                let packet = self.receive_message(header, size);

                if packet.is_err() {
                    println!("Expected {:?} bytes but the incoming data only has {:?}",
                             size, packet.err());
                    continue
                }

                let packet = packet.unwrap();

                println!("Received packet {} ({:?} bytes)", header, size);
            }
        });
    }

    fn receive_header(&mut self) -> u8 {
        const HEADER_SIZE: usize = 1;

        let mut header = [0u8; HEADER_SIZE];
        self.stream.read(&mut header).unwrap();

        header[0]
    }

    fn receive_message(&mut self, header: u8, size: usize) -> Result<Vec<u8>, usize> {
        let mut reader = io::BufReader::new(&mut self.stream);

        let mut received = vec![0u8; size];
        reader.read_exact(&mut received).unwrap();

        if received.len() < size {
            println!("Requested {} bytes but the stream only has {}", size, reader.capacity());
            return Err(reader.capacity())
        }

        Ok(received)
    }

    fn send_bytes(&mut self, data: Vec<u8>) -> () {
        self.stream.write(&*data).unwrap();
    }

}


impl Receiver for Net {

    fn receive_packet(&mut self, packet: Box<dyn Packet>) -> Result<Box<dyn Packet>, ()> {
        println!("Received packet {}", packet);

        Err(())
    }
}
