# Forza_dataout_parse
This is a rust library specifically designed for parsing Forza DataOut output messages, currently supporting parsing of messages from the Forza MotorSport Dash.

## Install
### Edit Cargo.toml
1. edit your `Cargo.toml`
2. add `forza_dataout_parse` follow `[dependencies]`. like:
   ```toml
   [dependencies]
   # ...
   forza_dataout_parse = "x"
   ```
3. exec shell
   ```shell
   cargo update
   ```
### OR Use shell command
```shell
cd /your/project/path
cargo add forza_dataout_parse
```

## How to use
This is a simple demo to listen ForzaMotorsport UPD.
```rust
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
extern crate forza_dataout_parse;
use forza_dataout_parse::dash_parser;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5300").expect("Couldn't bind to address");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((amt, _)) => {                
                let datagram = dash_parser::parse(&buffer[..amt]);
                
                if datagram.is_race_on == 1 {
                    print!("\rspeed = {:#?}, current_lap={:#?}", datagram.get_speed_by_kmh(), datagram.current_lap);
                    // println!("{:#?}", datagram.accel);
                    let _ = handle.flush();
                }
                thread::sleep(Duration::from_micros(10));
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                thread::sleep(Duration::from_micros(10));
                continue;
            }
        }
    }
}
```

## feature map
- [X] 😄support to_json() and serialization power by serde @0.1.1
- [x] 👌basic parse for dash datagram @0.1.0