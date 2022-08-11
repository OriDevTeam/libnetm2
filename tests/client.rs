// Relative Modules
mod test;

// Standard Uses

// Crate Uses
use libnetm2::network::{
    connection::settings::ConnectionSettings,
    vials::{client::Client}
};

// External Uses


pub fn create_client() -> Client {
    let mut client = Client::new();

    let authentication_channel = client.connect_channel(
        ConnectionSettings {
            ipv4: "192.168.1.106".to_string(),
            port: 20000 }
    );

    authentication_channel.prepare();

    client
}


#[test]
pub fn test_connection(){
    let client = create_client();
    let authentication_channel = client.channels.last().unwrap();

    assert_eq!(authentication_channel.connected(), true);
}


#[test]
pub fn test_login() {
    let client = create_client();
    let authentication_channel = client.channels.last().unwrap();

    /*
    let authentication_packet = LoginPacket {
        header: 1,
        username: "test".to_string(),
        password: "test".to_string()
    };

    authentication_channel.send_packet(authentication_packet);
    */

    assert_eq!(authentication_channel.connected(), true);
}
