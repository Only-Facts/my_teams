use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::{SocketAddr, TcpListener},
    sync::atomic::Ordering,
};

use crate::{client::Client, models::Database};

pub struct Server {
    listener: TcpListener,
    clients: HashMap<SocketAddr, Client>,
    db: Database,
}

impl Server {
    pub fn new(port: u16) -> std::io::Result<Self> {
        let address = format!("0.0.0.0:{port}");
        let listener = TcpListener::bind(&address)?;
        listener.set_nonblocking(true)?;

        let mut db = Database::default();
        db.load_from_file("myteams.data").ok();

        Ok(Server {
            listener,
            clients: HashMap::new(),
            db,
        })
    }

    fn accept_new_clients(&mut self) {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                if stream.set_nonblocking(true).is_ok() {
                    self.clients.insert(addr, Client::new(stream));
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => println!("Error accepting new client: {e}"),
        }
    }

    fn handle_command(&mut self, addr: SocketAddr, command_line: &str) {
        let parts: Vec<&str> = command_line.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "/login" => {
                if let Some(client) = self.clients.get_mut(&addr) {
                    client.queue_message("200 Login OK");
                }
            }
            _ => {
                if let Some(client) = self.clients.get_mut(&addr) {
                    client.queue_message("400 Unknown Command");
                }
            }
        }
    }

    fn process_clients(&mut self) {
        let mut disconnected = Vec::new();
        let mut commands_to_process = Vec::new();

        for (addr, client) in self.clients.iter_mut() {
            let mut buffer = [0; 2048];
            match client.stream.read(&mut buffer) {
                Ok(0) => disconnected.push(*addr),
                Ok(n) => {
                    client.read_buffer.extend_from_slice(&buffer[..n]);
                    while let Some(cmd) = client.extract_command() {
                        commands_to_process.push((*addr, cmd));
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(_) => disconnected.push(*addr),
            }
        }

        for (addr, cmd) in commands_to_process {
            self.handle_command(addr, &cmd);
        }

        for (addr, client) in self.clients.iter_mut() {
            if !client.write_buffer.is_empty() {
                match client.stream.write(&client.write_buffer) {
                    Ok(n) => {
                        client.write_buffer.drain(..n);
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                    Err(_) => disconnected.push(*addr),
                }
            }
        }

        for addr in disconnected {
            if let Some(client) = self.clients.remove(&addr) {
                // TODO: FFi call to server_event_user_logged_out if the client was logged (62)
                let _ = client.stream.shutdown(std::net::Shutdown::Both);
            }
        }
    }

    pub fn run(&mut self) {
        println!("Server listening...");

        while my_teams::ffi::RUNNING.load(Ordering::SeqCst) {
            self.accept_new_clients();
            self.process_clients();

            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        println!("\nShutting down server. Saving data...");
        if let Err(e) = self.db.save_to_file("myteams.data") {
            println!("Error saving data: {e}");
        }
    }
}
