use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;


const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("erreur de binding");
    server.set_nonblocking(true).expect("erreur pour set le mode non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept(){
            println!("client {} connecté", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("erreur de clonnage du client"));

            thread::spawn(move || loop {
                let mut buff = vec![0;MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_)=>{
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("format uutf8 invalid");

                        println!("{}: {:?}", addr, msg);
                    }
                   Err(ref err) if err.kind() == ErrorKind::WouldBlock =>(),
                    Err(_) => {
                        println!("arret de la connection avec {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client|{
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()

            }).collect::<Vec<_>>();
        }

        sleep();
    }
}
