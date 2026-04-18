use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    time::UNIX_EPOCH,
};

pub fn generate_uuid() -> String {
    let time = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{time:x}")
}

#[derive(Clone)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub is_connected: bool,
}

#[derive(Clone)]
pub struct Message {
    pub sender_uuid: String,
    pub receiver_uuid: String,
    pub body: String,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct Thread {
    pub uuid: String,
    pub title: String,
    pub message: String,
    pub author_uuid: String,
    pub replies: Vec<Message>,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct Channel {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub threads: HashMap<String, Thread>,
}

#[derive(Clone)]
pub struct Team {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub subscribers: Vec<String>,
    pub channels: HashMap<String, Channel>,
}

#[derive(Default)]
pub struct Database {
    pub users: HashMap<String, User>,
    pub teams: HashMap<String, Team>,
    pub private_messages: Vec<Message>,
}

impl Database {
    pub fn save_to_file(&self, filepath: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filepath)?;

        for user in self.users.values() {
            writeln!(
                file,
                "USER|{}|{}|{}",
                user.uuid, user.name, user.is_connected
            )?;
        }

        for msg in &self.private_messages {
            let safe_body = msg.body.replace('\n', "\\n");
            writeln!(
                file,
                "PM|{}|{}|{}|{}",
                msg.sender_uuid, msg.receiver_uuid, msg.timestamp, safe_body
            )?;
        }

        // TODO: Implement Teams, Channels & Threads too

        println!("Data saved successfully in {filepath}");
        Ok(())
    }

    pub fn load_from_file(&mut self, filepath: &str) -> std::io::Result<()> {
        let file = File::open(filepath);
        if file.is_err() {
            println!("file {filepath} not found.");
            return Ok(());
        }

        let reader = BufReader::new(file?);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "USER" if parts.len() == 4 => {
                    let user = User {
                        uuid: parts[1].to_string(),
                        name: parts[2].to_string(),
                        is_connected: false,
                    };
                    my_teams::ffi::call_user_loaded(&user.uuid, &user.name);
                    self.users.insert(user.uuid.clone(), user);
                }
                "PM" if parts.len() == 5 => {
                    let msg = Message {
                        sender_uuid: parts[1].to_string(),
                        receiver_uuid: parts[2].to_string(),
                        timestamp: parts[3].parse().unwrap_or(0),
                        body: parts[4].replace("\\n", "\n"),
                    };
                    self.private_messages.push(msg);
                }
                _ => {} // TODO: Implement Teams, Channels & Threads too
            }
        }

        println!("Data loaded successfully from {filepath}");
        Ok(())
    }
}
