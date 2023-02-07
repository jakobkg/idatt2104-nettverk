use std::{
    net::{Ipv4Addr, UdpSocket},
    process::exit,
};

fn main() {
    let socket = match UdpSocket::bind("224.0.0.7:5007") {
        Ok(socket) => socket,
        Err(error) => {
            eprintln!("Kunne ikke lytte til multicast, feil: {}", error);
            exit(1);
        }
    };

    socket
        .join_multicast_v4(&Ipv4Addr::new(224, 0, 0, 7), &Ipv4Addr::new(0, 0, 0, 0))
        .unwrap();

    println!("Abonnerer på multicast-addresse 224.0.0.7");

    loop {
        let mut msgbuf = [0u8; 1024];
        match socket.recv(&mut msgbuf) {
            Ok(len) => {
                println!("Mottok melding {}", String::from_utf8_lossy(&msgbuf[..len]))
            }
            Err(_) => {
                println!("Kunne ikke motta melding");
                exit(1);
            }
        }
    }
}
