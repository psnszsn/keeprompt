use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};

use std::{process, time};

use popol;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{io, net};

// struct Listener {
//     path: PathBuf,
//     listener: UnixListener,
// }

// impl Listener {
//     fn bind(path: impl AsRef<Path>) -> std::io::Result<Self> {
//         let path = path.as_ref().to_owned();
//         let _ = std::fs::remove_file(&path).unwrap();
//         UnixListener::bind(&path).map(|listener| Listener { path, listener })
//     }
// }

// impl Drop for Listener {
//     fn drop(&mut self) {
//         // There's no way to return a useful error here
//         let _ = std::fs::remove_file(&self.path).unwrap();
//     }
// }

// impl std::ops::Deref for Listener {
//     type Target = UnixListener;

//     fn deref(&self) -> &Self::Target {
//         &self.listener
//     }
// }

fn handle_client(stream: &UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{:#?}", line);
    }
}

pub fn connect() -> io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/rust-uds.sock")?;
    stream.write(b"pingg")?;
    Ok(())
}

pub fn run(config: &super::Config, pwds: &super::PwdsHM) -> io::Result<()> {
    let path = Path::new("/tmp/rust-uds.sock");
    if path.exists() {
        let _ = std::fs::remove_file(path).unwrap();
    }
    let listener = UnixListener::bind(path).unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut sources = popol::Sources::new();
    let mut events = popol::Events::new();

    let mut peers: Vec<std::os::unix::net::UnixStream> = Vec::new();

    #[derive(Eq, PartialEq, Clone)]
    enum Source {
        Peer(usize),
        Listener,
    }

    sources.register(Source::Listener, &listener, popol::interest::READ);

    loop {
        // sources.wait(&mut events)?;
        match sources.wait_timeout(&mut events, time::Duration::from_secs(60)) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::TimedOut => process::exit(1),
            Err(err) => return Err(err),
        }

        for (key, event) in events.iter() {
            match key {
                Source::Listener => loop {
                    match listener.accept() {
                        Ok((conn, addr)) => {
                            println!("Got a client: {:?}", addr);
                            // handle_client(socket);
                            conn.set_nonblocking(true)?;
                            sources.register(
                                Source::Peer(peers.len()),
                                &conn,
                                popol::interest::READ,
                            );

                            peers.push(conn);
                        }
                        Err(e) => {
                            if e.kind() == io::ErrorKind::WouldBlock {
                                break;
                            }
                            println!("accept function failed: {:?}", e);
                            return Err(e);
                        }
                    }
                },
                Source::Peer(index) => {
                    if event.readable {
                        println!("{} has data to be read", index);
                        // handle_client(peers.get(0).unwrap())
                        let s = peers.get(*index).unwrap();
                        let mut b = std::io::BufReader::new(s);
                        let mut ss = String::new();
                        // let len = b.read_to_string(&mut ss).unwrap_or(1);
                        let len = b.read_line(&mut ss).unwrap();
                        if len == 0 {
                            sources.unregister(key);
                        }
                        println!("read {:#?} ", ss);
                        if ss == "pingg" {
                            super::select_pwd(config, pwds);
                        }
                    }
                    // if event.writable {
                    //     println!("{} has data to be written", "asd");
                    // }
                }
            }
        }
    }
}
