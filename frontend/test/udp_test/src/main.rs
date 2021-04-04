use std::net::UdpSocket;
use std::io::{self, Read};

//this code is probably a bit bad because I don't know rust but it works :) - Luca

fn main() -> std::io::Result<()> {
    {
            let mut socket = UdpSocket::bind("127.0.0.1:59828")?;

            let mut buf = [0;256];
            buf[3] = 77;

            let mut src = "127.0.0.1:59827";
            socket.send_to(&buf, &src)?;
    }
    Ok(())
}
