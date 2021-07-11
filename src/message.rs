

pub struct Message {
    buffer: [u8; 1024],
}


impl Message {

    pub fn new(buffer: [u8; 1024]) -> Self {
        Self {
            buffer
        }
    }

    pub fn display(self) {
        println!("Received message: {}", String::from_utf8_lossy(&self.buffer));
    }
}
