use datachannel::{Connection, ConnectionConfig, DataChannelHandler};
use std::sync::{Arc, Mutex};

fn main() {
    let config = ConnectionConfig::default();
    let conn = Connection::new(&config).unwrap();

    conn.on_data_channel(|channel| {
        let handler = Arc::new(Mutex::new(AudioChannelHandler {}));
        channel.set_handler(handler.clone());
    });

    println!("🎧 WebRTC Audio Streaming Server Running!");
}

struct AudioChannelHandler;

impl DataChannelHandler for AudioChannelHandler {
    fn on_message(&self, data: &[u8]) {
        println!("🎵 Streaming Audio: {} bytes received", data.len());
    }
}
