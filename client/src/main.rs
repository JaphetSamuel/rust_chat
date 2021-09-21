use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("erreur de connection au stream");
    client.set_nonblocking(true).expect("erreur de non_blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while((|&x| x != 0)).collect::<Vec<_>>();
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock =>(),
            Err(_) => {
                println!("connection avec le server servered");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) =>{
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("ecriture au socket echoue");
                println!("message envoye: {}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("Entrez un message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("erreur de lecture de l'entre std");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }

    println!("au revoir");
}
