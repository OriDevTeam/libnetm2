// Sustem Usings

// Crate Usings
use super::connection::channel::Channel;


// External Usings



pub struct Client {

    pub channels: Channel

}


impl Client {
    
    #[no_mangle]
    pub extern fn connect() -> bool {
        todo!()
    }

    #[no_mangle]
    pub fn disconnect() -> bool {
        todo!()        
    }
}

