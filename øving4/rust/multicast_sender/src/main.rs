use std::{net::UdpSocket, process::exit};

fn main() {
    let socket = match UdpSocket::bind("224.0.0.7:5008") {
        Ok(socket) => socket,
        Err(error) => {
            eprintln!("Kunne ikke koble til multicast, feil: {}", error);
            exit(1);
        }
    };

    match socket.send_to("Hallo".as_bytes(), "224.0.0.7:5007") {
        Ok(_) => {
            println!("Melding sendt!");
        }
        Err(err) => {
            println!("Kunne ikke sende :(");
            println!("{err}");
        }
    };
}
